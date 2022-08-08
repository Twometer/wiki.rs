use std::{fs::File, time::Instant};

mod index;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let index_file =
        File::open(r"X:\Backups\Wikipedia\enwiki-latest-pages-articles-multistream-index.txt")?;
    let index = index::Index::load(index_file)?;

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

    Ok(())
}
