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
}

impl IndexEntry {
    fn parse(line: &str) -> anyhow::Result<Self> {
        let mut separators = [0usize; 2];
        let mut index = 0;

        for (i, c) in line.char_indices() {
            if c == ':' {
                separators[index] = i;
                index += 1;
            }

            if index >= 2 {
                break;
            }
        }

        let [sep0, sep1] = separators;

        let offset = &line[0..sep0];
        let page_id = &line[sep0 + 1..sep1];
        let page_name = &line[sep1 + 1..];

        let offset: u64 = offset.parse()?;
        let page_id: u64 = page_id.parse()?;
        let page_name = page_name.to_owned();

        return Ok(Self {
            offset,
            page_id,
            page_name,
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
            .filter(|entry| Self::starts_with(&entry.page_name, &query))
            .take(100)
            .collect::<Vec<&IndexEntry>>();

        results.sort_by(|a, b| a.page_name.len().cmp(&b.page_name.len()));
        results
    }

    pub fn find_article_exact(&self, name: &str) -> Option<&IndexEntry> {
        self.entries
            .par_iter()
            .find_any(|entry| Self::equals(&entry.page_name, name))
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    fn equals(name: &str, query: &str) -> bool {
        if name.len() != query.len() {
            return false;
        }

        for (left, right) in name.chars().zip(query.chars()) {
            if !left.to_lowercase().eq(right.to_lowercase()) {
                return false;
            }
        }

        true
    }

    fn starts_with(name: &str, query: &str) -> bool {
        let mut name = name.chars();
        for query_chr in query.chars() {
            match name.next() {
                Some(name_chr) => {
                    if !name_chr.to_lowercase().eq(query_chr.to_lowercase()) {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }
}
