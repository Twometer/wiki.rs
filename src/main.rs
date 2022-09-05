// #![windows_subsystem = "windows"]

use std::time::Instant;

use anyhow::bail;
use thiserror::Error;
use url::Url;
use urlencoding::decode;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    http::ResponseBuilder,
    webview::WebViewBuilder,
};

use crate::{render::render_article, resource::ResourceManager};

mod render;
mod resource;
mod template;
mod wiki;

#[derive(Debug)]
enum ParsedUrl {
    Resource(String),
    Article(String),
}

#[derive(Error, Debug)]
pub enum UrlError {
    #[error("url namespace not found")]
    UnknownNamespace,

    #[error("url path is incomplete")]
    IncompletePath,
}

fn parse_url(url: &str) -> anyhow::Result<ParsedUrl> {
    let url = Url::parse(url)?;
    let mut path = url.path_segments().unwrap();

    let mut next_path_part = move || return path.next().ok_or(UrlError::IncompletePath);

    let namespace = decode(next_path_part()?)?;
    let name = decode(next_path_part()?)?.to_string();

    Ok(match &*namespace {
        "res" => ParsedUrl::Resource(name),
        "article" => ParsedUrl::Article(name),
        _ => bail!(UrlError::UnknownNamespace),
    })
}

fn main() -> anyhow::Result<()> {
    println!("Starting up wiki.rs ...");

    let index = wiki::index::Index::from_file(
        r"X:\Backups\Wikipedia\enwiki-latest-pages-articles-multistream-index.txt",
    )?;
    let article_db = wiki::article::ArticleDatabase::from_file(
        r"X:\Backups\Wikipedia\enwiki-latest-pages-articles-multistream.xml.bz2",
    )?;

    let mut resources = ResourceManager::new();
    resources.register("article.html", include_bytes!("../res/article.html"));
    resources.register("styles.css", include_bytes!("../res/styles.css"));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("wiki.rs")
        .build(&event_loop)?;

    let mut _web_view = WebViewBuilder::new(window)?
        .with_custom_protocol("local".into(), move |request| {
            println!("Handling local request for {}", request.uri());
            let url = parse_url(request.uri());

            if let Ok(url) = url {
                match url {
                    ParsedUrl::Article(name) => {
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
                        let article_html = render_article(&resources, &article_data);
                        println!("Rendered article in {:.2?}", time.elapsed());

                        ResponseBuilder::new()
                            .mimetype("text/html")
                            .body(article_html.into_bytes())
                    }
                    ParsedUrl::Resource(name) => {
                        if let Some(resource) = resources.find_resource(&name) {
                            return resource.into();
                        }

                        ResponseBuilder::new()
                            .mimetype("text/plain")
                            .body("not found".to_string().into_bytes())
                    }
                }
            } else {
                ResponseBuilder::new()
                    .mimetype("text/plain")
                    .body("not found".to_string().into_bytes())
            }
        })
        .with_url("local://wiki.rs/article/2005 United Kingdom general election in England")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Started wry window"),
            Event::MenuEvent { menu_id, .. } => match menu_id {
                action_exit => {
                    *control_flow = ControlFlow::Exit;
                }
                action_search => {}
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
