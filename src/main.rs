use std::time::Instant;

mod article;
mod index;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let index = index::Index::from_file(
        r"X:\Backups\Wikipedia\enwiki-latest-pages-articles-multistream-index.txt",
    )?;
    let article_db = article::ArticleDatabase::from_file(
        r"X:\Backups\Wikipedia\enwiki-latest-pages-articles-multistream.xml.bz2",
    )?;

    println!(
        "Loaded index with {} entries in {:.2?}",
        index.size(),
        now.elapsed()
    );

    let now = Instant::now();
    let search_results = index.find_article("Rust");
    println!(
        "Search returned {} results in {:.2?}",
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
        .find_article_exact("United Kingdom general election, 2005 (Bristol)")
        .expect("Test article not found");

    println!(
        "Found exact article {} at {}/{} in {:.2?}",
        exact.page_name,
        exact.offset,
        exact.page_id,
        now.elapsed()
    );

    let now = Instant::now();
    let article_data = article_db.get_article(exact)?;
    println!("Loaded article data from DB in {:.2?}", now.elapsed());

    dbg!(article_data);

    Ok(())
}
