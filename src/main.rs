use std::{fs::File, io::Write, time::Instant};

use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{MenuBar, MenuItemAttributes},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

use crate::render::render_article;

mod article;
mod index;
mod render;
mod template;

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

    // Test redirect article: United Kingdom general election, 2005 (Bristol)

    let now = Instant::now();
    let exact = index
        .find_article_exact("2005 United Kingdom general election in England")
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

    let now = Instant::now();
    let html = render_article(&article_data);
    println!("Rendered in {:.2?}", now.elapsed());

    //let mut file = File::create("work/test.html")?;
    //file.write_all(html.as_bytes())?;

    println!(">> Done");

    let mut menu = MenuBar::new();
    menu.add_item(MenuItemAttributes::new("Test"));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wiki.rs")
        .with_menu(menu)
        .build(&event_loop)?;

    let web_view = WebViewBuilder::new(window)?.with_html(html)?.build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
