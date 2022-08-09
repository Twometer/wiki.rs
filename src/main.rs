// #![windows_subsystem = "windows"]

use std::{borrow::Cow, collections::HashMap, time::Instant};

use url::Url;
use urlencoding::decode;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{MenuBar, MenuItemAttributes},
        window::WindowBuilder,
    },
    http::ResponseBuilder,
    webview::WebViewBuilder,
};

use crate::render::render_article;

mod article;
mod index;
mod render;
mod template;

fn main() -> anyhow::Result<()> {
    println!("Starting up wiki.rs ...");

    let index = index::Index::from_file(
        r"X:\Backups\Wikipedia\enwiki-latest-pages-articles-multistream-index.txt",
    )?;
    let article_db = article::ArticleDatabase::from_file(
        r"X:\Backups\Wikipedia\enwiki-latest-pages-articles-multistream.xml.bz2",
    )?;

    let mut menu = MenuBar::new();
    menu.add_item(MenuItemAttributes::new("Test"));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wiki.rs")
        .with_menu(menu)
        .build(&event_loop)?;

    let _web_view = WebViewBuilder::new(window)?
        .with_custom_protocol("wiki".into(), move |request| {
            println!("Handling request to {}", request.uri());

            let url = Url::parse(request.uri()).unwrap();
            let mut path = url.path_segments().unwrap();

            let name = decode(path.nth(0).unwrap()).unwrap();

            println!("Loading article {}", name);

            let time = Instant::now();
            let article = index.find_article_exact(&name);
            if article.is_none() {
                return ResponseBuilder::new()
                    .mimetype("text/plain")
                    .body("not found".to_string().into_bytes());
            }

            let article = article.unwrap();
            println!("Located article in {:.2?}", time.elapsed());

            let time = Instant::now();
            let article_data = article_db.get_article(&article).unwrap();
            println!("Extracted article in {:.2?}", time.elapsed());

            let time = Instant::now();
            let article_html = render_article(&article_data);
            println!("Rendered article in {:.2?}", time.elapsed());

            ResponseBuilder::new()
                .mimetype("text/html")
                .body(article_html.into_bytes())
        })
        .with_url("wiki://page/2005 United Kingdom general election in England")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Started wry window"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
