use std::collections::HashMap;

use parse_wiki_text::{Configuration, Node};

use crate::{article::Article, template::render_template};

pub fn render_article(article: &Article) -> String {
    let mut render_ctx = RenderContext::new();
    render_ctx.render_article_body(article);

    let template = include_str!("../res/article.html");
    render_template(
        template,
        HashMap::from([
            ("title", article.title.as_str()),
            ("body", render_ctx.html.as_str()),
        ]),
    )
}

struct RenderContext {
    html: String,
    is_italic: bool,
    is_bold: bool,
    is_bold_italic: bool,
    is_comment: bool,
}

impl RenderContext {
    fn new() -> Self {
        Self {
            html: String::new(),
            is_italic: false,
            is_bold: false,
            is_bold_italic: false,
            is_comment: false,
        }
    }

    fn render_article_body(&mut self, article: &Article) {
        let root = Configuration::default().parse(article.body.as_str());

        self.html.clear();
        self.render_nodes(&root.nodes);
    }

    fn render_nodes(&mut self, nodes: &Vec<Node>) {
        for node in nodes {
            self.render_node(&node);
        }
    }

    fn render_node(&mut self, node: &Node) {
        match node {
            parse_wiki_text::Node::Bold { .. } => {
                if !self.is_bold {
                    self.html.push_str("<strong>");
                } else {
                    self.html.push_str("</strong>");
                }
                self.is_bold = !self.is_bold;
            }
            parse_wiki_text::Node::BoldItalic { .. } => {
                if !self.is_bold_italic {
                    self.html.push_str("<strong><em>");
                } else {
                    self.html.push_str("</em></strong>");
                }
                self.is_bold_italic = !self.is_bold_italic;
            }
            parse_wiki_text::Node::Italic { .. } => {
                if !self.is_italic {
                    self.html.push_str("<em>");
                } else {
                    self.html.push_str("</em>");
                }
                self.is_italic = !self.is_italic;
            }
            parse_wiki_text::Node::Comment { .. } => {
                if !self.is_comment {
                    self.html.push_str("<!--");
                } else {
                    self.html.push_str("-->");
                }
                self.is_comment = !self.is_comment;
            }
            parse_wiki_text::Node::Category { target, .. } => {
                self.html.push_str(&format!(
                    r#"<a class="category" href="wiki://{}">{}</a>"#,
                    target, target
                ));
            }
            parse_wiki_text::Node::CharacterEntity { character, .. } => {
                self.html.push(*character);
            }
            parse_wiki_text::Node::DefinitionList { items, .. } => {
                self.html.push_str("<dl>");
                for itm in items {
                    self.render_nodes(&itm.nodes);
                }
                self.html.push_str("</dl>");
            }
            parse_wiki_text::Node::StartTag { name, .. } => {
                self.html.push_str(&format!("<{}>", name));
            }
            parse_wiki_text::Node::EndTag { name, .. } => {
                self.html.push_str(&format!("</{}>", name));
            }
            parse_wiki_text::Node::Heading { level, nodes, .. } => {
                self.html.push_str(&format!("<h{}>", level));
                self.render_nodes(nodes);
                self.html.push_str(&format!("</h{}>", level));
            }
            parse_wiki_text::Node::HorizontalDivider { .. } => {
                self.html.push_str("</hr>");
            }
            parse_wiki_text::Node::Image { target, text, .. } => {
                self.html
                    .push_str(&format!(r#"<figure><img src="{}"/><figcaption>"#, target));
                self.render_nodes(text);
                self.html.push_str("</figcaption></figure>");
            }
            parse_wiki_text::Node::Link { target, text, .. } => {
                self.html
                    .push_str(&format!("<a href=\"wiki://{}\">", target));
                self.render_nodes(text);
                self.html.push_str("</a>");
            }
            parse_wiki_text::Node::Redirect { target, .. } => {
                self.html
                    .push_str(&format!("<a href=\"wiki://{}\">Redirect</a>", target));
            }
            parse_wiki_text::Node::ExternalLink { nodes, .. } => {
                self.html.push_str("<a href=\"#\">");
                self.render_nodes(nodes);
                self.html.push_str("</a>");
            }
            parse_wiki_text::Node::MagicWord { .. } => {}
            parse_wiki_text::Node::ParagraphBreak { .. } => {
                self.html.push_str("<p/>");
            }
            parse_wiki_text::Node::Parameter { .. } => {
                // TODO: Insert a parameter when inside of a template call
            }
            parse_wiki_text::Node::Template { .. } => {
                // TODO: Insert a template from the Template: namespace, or if the template
                //       is {{#invoke:$name|$arg1|...}}, it's a Lua template from the Module: namespace
            }
            parse_wiki_text::Node::Preformatted { nodes, .. } => {
                self.html.push_str("<pre>");
                self.render_nodes(nodes);
                self.html.push_str("</pre>");
            }
            parse_wiki_text::Node::Table {
                attributes,
                captions,
                rows,
                ..
            } => {
                self.html.push_str("<table ");
                self.render_nodes(attributes);
                self.html.push_str(">");

                self.html.push_str("<thead><tr>");
                for cap in captions {
                    self.html.push_str("<th ");
                    if let Some(attributes) = cap.attributes.as_ref() {
                        self.render_nodes(attributes);
                    }
                    self.html.push_str(">");

                    self.render_nodes(&cap.content);
                    self.html.push_str("</th>");
                }
                self.html.push_str("</tr></thead>");

                self.html.push_str("<tbody>");
                for row in rows {
                    self.html.push_str("<tr ");
                    self.render_nodes(&row.attributes);
                    self.html.push_str(">");

                    for cell in &row.cells {
                        self.html.push_str("<td ");
                        if let Some(attributes) = cell.attributes.as_ref() {
                            self.render_nodes(attributes);
                        }
                        self.html.push_str(">");

                        self.render_nodes(&cell.content);
                        self.html.push_str("</td>");
                    }
                    self.html.push_str("</tr>");
                }
                self.html.push_str("</tbody></table>");
            }
            parse_wiki_text::Node::Tag { name, nodes, .. } => {
                self.html.push_str(&format!("<{}>", name));
                self.render_nodes(nodes);
                self.html.push_str(&format!("</{}>", name));
            }
            parse_wiki_text::Node::Text { value, .. } => {
                self.html.push_str(value);
            }
            parse_wiki_text::Node::OrderedList { items, .. } => {
                self.html.push_str("<ol>");
                for itm in items {
                    self.html.push_str("<li>");
                    self.render_nodes(&itm.nodes);
                    self.html.push_str("</li>");
                }
                self.html.push_str("</ol>");
            }
            parse_wiki_text::Node::UnorderedList { items, .. } => {
                self.html.push_str("<ul>");
                for itm in items {
                    self.html.push_str("<li>");
                    self.render_nodes(&itm.nodes);
                    self.html.push_str("</li>");
                }
                self.html.push_str("</ul>");
            }
        }
    }
}
