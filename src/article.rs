use std::{fs::File, io};

use anyhow::{bail, Ok};
use bzip2_rs::DecoderReader;
use chrono::{DateTime, Utc};
use memmap::{Mmap, MmapOptions};
use minidom::Element;
use thiserror::Error;

use crate::index::IndexEntry;

pub struct ArticleDatabase {
    data: Mmap,
}

#[derive(Debug, Clone)]
pub struct Article {
    pub id: u64,
    pub title: String,
    pub last_changed_at: DateTime<Utc>,
    pub last_changed_by: String,
    pub body: String,
}

#[derive(Error, Debug)]
pub enum ArticleError {
    #[error("requested article not found")]
    ArticleNotFound,

    #[error("missing property on page")]
    MissingProperty(String),
}

impl ArticleDatabase {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        Self::load(File::open(path)?)
    }

    pub fn load(file: File) -> anyhow::Result<Self> {
        Ok(Self {
            data: unsafe { MmapOptions::new().map(&file)? },
        })
    }

    pub fn get_article(&self, idx: &IndexEntry) -> anyhow::Result<Article> {
        let chunk = self.get_article_chunk(idx)?;
        let root: Element = chunk.parse()?;
        let article = root.children().find(|ch| {
            let option = ch.get_child("id", "");
            if let Some(id) = option {
                id.text() == idx.page_id.to_string()
            } else {
                false
            }
        });

        match article {
            Some(article) => Self::parse_article(article),
            None => bail!(ArticleError::ArticleNotFound),
        }
    }

    fn get_article_chunk(&self, idx: &IndexEntry) -> anyhow::Result<String> {
        let offset = idx.offset as usize;
        let bzip_data = &self.data[offset..];

        let mut decoded = Vec::<u8>::new();
        let mut reader = DecoderReader::new(bzip_data);
        io::copy(&mut reader, &mut decoded)?;

        let raw_xml_pages = String::from_utf8(decoded)?;
        let reconstructed_xml = format!("<pages xmlns=\"\">{}</pages>", &raw_xml_pages);

        Ok(reconstructed_xml)
    }

    fn parse_article(article: &Element) -> anyhow::Result<Article> {
        let id: u64 = article.try_get_child("id")?.text().parse()?;
        let title = article.try_get_child("title")?.text();

        let revision = article.try_get_child("revision")?;
        let body = revision.try_get_child("text")?.text();

        let last_changed_at: DateTime<Utc> = revision.try_get_child("timestamp")?.text().parse()?;
        let last_changed_by = revision
            .try_get_child("contributor")?
            .try_get_child("username")?
            .text();

        Ok(Article {
            id,
            title,
            last_changed_by,
            last_changed_at,
            body,
        })
    }
}

trait TryGetChild {
    fn try_get_child(&self, name: &str) -> anyhow::Result<&Element>;
}

impl TryGetChild for &Element {
    fn try_get_child(&self, name: &str) -> anyhow::Result<&Element> {
        let child = self.get_child(name, "");
        match child {
            Some(child) => Ok(child),
            None => bail!(ArticleError::MissingProperty(name.to_owned())),
        }
    }
}
