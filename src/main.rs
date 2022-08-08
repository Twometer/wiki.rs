use std::{fs::File, io, time::Instant};

use bzip2_rs::DecoderReader;
use memmap::MmapOptions;

mod index;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let index = index::Index::from_file(
        r"X:\Backups\Wikipedia\enwiki-latest-pages-articles-multistream-index.txt",
    )?;

    println!(
        "Found {} pages in index after {:.2?}",
        index.size(),
        now.elapsed()
    );

    let now = Instant::now();
    let search_results = index.find_article("Rust");
    println!(
        "Got {} search results after {:.2?}",
        search_results.len(),
        now.elapsed()
    );

    let article = search_results
        .first()
        .expect("No results found for test query");

    println!(
        "Found article {} at {}/{}",
        article.page_name, article.offset, article.page_id
    );

    let now = Instant::now();
    let exact = index
        .find_exact("United Kingdom general election, 2005 (Bristol)")
        .expect("Test article not found");

    println!(
        "Found exact article {} at {}/{} after {:.2?}",
        exact.page_name,
        exact.offset,
        exact.page_id,
        now.elapsed()
    );

    let now = Instant::now();
    let articles_file =
        File::open(r"X:\Backups\Wikipedia\enwiki-latest-pages-articles-multistream.xml.bz2")?;
    let mmap = unsafe { MmapOptions::new().map(&articles_file)? };

    let offset = exact.offset as usize;
    let bzip_stream = &mmap[offset..];

    let mut output = File::create("article.xml")?;
    let mut reader = DecoderReader::new(bzip_stream);
    io::copy(&mut reader, &mut output)?;
    println!("Decoded article after {:.2?}", now.elapsed());

    Ok(())
}
