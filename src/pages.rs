use kata::TemplateContext;

use crate::{renderer::ArticleRenderer, resource::ResourceManager, wiki::article::Article};

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
