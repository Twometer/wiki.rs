use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rayon::prelude::{ParallelBridge, ParallelIterator};

pub struct IndexEntry {
    pub offset: u64,
    pub page_id: u64,
    pub page_name: String,
}

impl IndexEntry {
    fn parse(line: &str) -> anyhow::Result<Self> {
        let mut parts = line.split(":");

        let offset: u64 = parts.next().unwrap().parse()?;
        let page_id: u64 = parts.next().unwrap().parse()?;
        let page_name = parts.next().unwrap().to_owned();

        return Ok(Self {
            offset,
            page_id,
            page_name,
        });
    }
}

pub fn from_file(file: File) -> Vec<IndexEntry> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .par_bridge()
        .filter_map(|line| IndexEntry::parse(&line).ok())
        .collect()
}
