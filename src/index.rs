use std::{fs::File, io::Read, str};

use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

pub struct Index {
    entries: Vec<IndexEntry>,
}

pub struct IndexEntry {
    pub offset: u64,
    pub page_id: u64,
    pub page_name: String,
    page_name_lowercase: String,
}

impl IndexEntry {
    fn parse(line: &str) -> anyhow::Result<Self> {
        let mut parts = line.split(":");

        let offset: u64 = parts.next().expect("Missing offset").parse()?;
        let page_id: u64 = parts.next().expect("Missing page ID").parse()?;
        let page_name = parts.next().expect("Missing page name").to_owned();
        let page_name_lowercase = page_name.to_lowercase();

        return Ok(Self {
            offset,
            page_id,
            page_name,
            page_name_lowercase,
        });
    }
}

impl Index {
    pub fn from_file(path: &str) -> anyhow::Result<Index> {
        let file = File::open(path)?;
        Self::load(file)
    }

    pub fn load(mut file: File) -> anyhow::Result<Index> {
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let lines: Vec<&str> = data.par_lines().collect();
        return Ok(Index {
            entries: lines
                .par_iter()
                .map(|line| IndexEntry::parse(line).expect("Failed to parse index entry"))
                .collect(),
        });
    }

    pub fn find_article(&self, query: &str) -> Vec<&IndexEntry> {
        let query = query.to_lowercase();

        let mut results = self
            .entries
            .iter()
            .filter(|entry| entry.page_name_lowercase.starts_with(&query))
            .take(100)
            .collect::<Vec<&IndexEntry>>();

        results.sort_by(|a, b| a.page_name.len().cmp(&b.page_name.len()));
        results
    }

    pub fn find_exact(&self, name: &str) -> Option<&IndexEntry> {
        self.entries
            .par_iter()
            .find_any(|entry| entry.page_name == name)
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
