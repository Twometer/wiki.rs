use kata::TemplateContext;

use crate::{
    renderer::ArticleRenderer,
    resource::ResourceManager,
    wiki::{article::Article, index::IndexEntry},
};

pub fn render_article_page(resources: &ResourceManager, article: &Article) -> String {
    let mut renderer = ArticleRenderer::new();
    renderer.render_article_body(article);

    let template = resources
        .find_template("article.html")
        .expect("Failed to find article template");

    let mut ctx = TemplateContext::new();
    ctx.set_str("body", renderer.html());
    ctx.set_str("title", &article.title);

    template
        .render(&ctx)
        .expect("Failed to render article template")
}

pub fn render_results_page(
    resources: &ResourceManager,
    query: &str,
    index_entries: &Vec<&IndexEntry>,
) -> String {
    let results: Vec<&str> = index_entries
        .iter()
        .map(|entry| entry.page_name.as_str())
        .collect();

    let template = resources
        .find_template("search.html")
        .expect("Failed to find search template");

    let mut ctx = TemplateContext::new();
    ctx.set_str("query", query);
    ctx.set_str_array("results", &results);

    template
        .render(&ctx)
        .expect("Failed to render search template")
}
