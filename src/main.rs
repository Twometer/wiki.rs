use std::fs::File;

mod index;

fn main() -> anyhow::Result<()> {
    let index_file =
        File::open(r"X:\Backups\Wikipedia\enwiki-latest-pages-articles-multistream-index.txt")?;
    let index = index::from_file(index_file);

    println!("Found {} pages in index", index.len());

    Ok(())
}
