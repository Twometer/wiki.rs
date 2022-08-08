use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rayon::{
    prelude::{IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    slice::ParallelSliceMut,
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

        let offset: u64 = parts.next().unwrap().parse()?;
        let page_id: u64 = parts.next().unwrap().parse()?;
        let page_name = parts.next().unwrap().to_owned();
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
    pub fn load(file: &File) -> Index {
        let reader = BufReader::new(file);
        let lines: Vec<Result<String, std::io::Error>> = reader.lines().collect();
        return Index {
            entries: lines
                .par_iter()
                .map(|line| IndexEntry::parse(line.as_ref().unwrap()).unwrap())
                .collect(),
        };
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

    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
