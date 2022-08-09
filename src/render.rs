use std::collections::HashMap;

use parse_wiki_text::{Configuration, Node};

use crate::{article::Article, template::render_template};

pub fn render_article(article: &Article) -> String {
    let mut renderer = ArticleRenderer::new();
    renderer.render_article_body(article);

    let template = include_str!("../res/article.html");
    render_template(
        template,
        HashMap::from([
            ("title", article.title.as_str()),
            ("body", renderer.html.as_str()),
        ]),
    )
}

struct ArticleRenderer {
    html: String,
    is_italic: bool,
    is_bold: bool,
    is_bold_italic: bool,
    is_comment: bool,
}

impl ArticleRenderer {
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
            Node::Bold { .. } => {
                if !self.is_bold {
                    self.open_tag("strong");
                } else {
                    self.close_tag("strong");
                }
                self.is_bold = !self.is_bold;
            }
            Node::BoldItalic { .. } => {
                if !self.is_bold_italic {
                    self.open_tag("strong");
                    self.open_tag("em");
                } else {
                    self.close_tag("em");
                    self.close_tag("strong");
                }
                self.is_bold_italic = !self.is_bold_italic;
            }
            Node::Italic { .. } => {
                if !self.is_italic {
                    self.open_tag("em");
                } else {
                    self.close_tag("em");
                }
                self.is_italic = !self.is_italic;
            }
            Node::Comment { .. } => {
                if !self.is_comment {
                    self.append("<!--");
                } else {
                    self.append("-->");
                }
                self.is_comment = !self.is_comment;
            }
            Node::Category { target, .. } => {
                self.append(&format!(
                    r#"<a class="category" href="/{}">{}</a>"#,
                    target, target
                ));
            }
            Node::CharacterEntity { character, .. } => {
                self.append_chr(*character);
            }
            Node::DefinitionList { items, .. } => {
                self.open_tag("dl");
                for itm in items {
                    self.render_nodes(&itm.nodes);
                }
                self.close_tag("dl");
            }
            Node::StartTag { name, .. } => {
                self.open_tag(&name);
            }
            Node::EndTag { name, .. } => {
                self.close_tag(&name);
            }
            Node::Heading { level, nodes, .. } => {
                let tag_name = format!("h{}", level);

                self.open_tag(&tag_name);
                self.render_nodes(nodes);
                self.close_tag(&tag_name);
            }
            Node::HorizontalDivider { .. } => {
                self.void_tag("hr");
            }
            Node::Image { target, text, .. } => {
                self.open_tag("figure");
                self.append(&format!(r#"<img src="{}"/>"#, target));

                self.open_tag("figcaption");
                self.render_nodes(text);
                self.close_tag("figcaption");
                self.close_tag("figure");
            }
            Node::Link { target, text, .. } => {
                self.append(&format!("<a href=\"/{}\">", target));
                self.render_nodes(text);
                self.append("</a>");
            }
            Node::Redirect { target, .. } => {
                self.append(&format!("<a href=\"/{}\">Redirect</a>", target));
            }
            Node::ExternalLink { nodes, .. } => {
                self.append(r##"<a href="#">"##);
                self.render_nodes(nodes);
                self.append("</a>");
            }
            Node::ParagraphBreak { .. } => {
                self.void_tag("p");
            }
            Node::MagicWord { .. } => {}
            Node::Parameter { .. } => {
                // TODO: Insert a parameter when inside of a template call
            }
            Node::Template { .. } => {
                // TODO: Insert a template from the Template: namespace, or if the template
                //       is {{#invoke:$name|$arg1|...}}, it's a Lua template from the Module: namespace
            }
            Node::Preformatted { nodes, .. } => {
                self.open_tag("pre");
                self.render_nodes(nodes);
                self.close_tag("pre");
            }
            Node::Table {
                attributes,
                captions,
                rows,
                ..
            } => {
                self.append("<table ");
                self.render_nodes(attributes);
                self.append(">");

                self.append("<thead><tr>");
                for cap in captions {
                    self.append("<th ");
                    if let Some(attributes) = cap.attributes.as_ref() {
                        self.render_nodes(attributes);
                    }
                    self.append(">");

                    self.render_nodes(&cap.content);
                    self.append("</th>");
                }
                self.append("</tr></thead>");

                self.append("<tbody>");
                for row in rows {
                    self.append("<tr ");
                    self.render_nodes(&row.attributes);
                    self.append(">");

                    for cell in &row.cells {
                        self.append("<td ");
                        if let Some(attributes) = cell.attributes.as_ref() {
                            self.render_nodes(attributes);
                        }
                        self.append(">");

                        self.render_nodes(&cell.content);
                        self.append("</td>");
                    }
                    self.append("</tr>");
                }
                self.append("</tbody></table>");
            }
            Node::Tag { name, nodes, .. } => {
                self.open_tag(&name);
                self.render_nodes(nodes);
                self.close_tag(&name);
            }
            Node::Text { value, .. } => {
                self.append(value);
            }
            Node::OrderedList { items, .. } => {
                self.open_tag("ol");
                for itm in items {
                    self.open_tag("li");
                    self.render_nodes(&itm.nodes);
                    self.close_tag("li");
                }
                self.close_tag("ol");
            }
            Node::UnorderedList { items, .. } => {
                self.open_tag("ul");
                for itm in items {
                    self.open_tag("li");
                    self.render_nodes(&itm.nodes);
                    self.close_tag("li");
                }
                self.close_tag("ul");
            }
        }
    }

    fn append(&mut self, data: &str) {
        self.html.push_str(data);
    }

    fn append_chr(&mut self, data: char) {
        self.html.push(data);
    }

    fn open_tag(&mut self, tag: &str) {
        self.append_chr('<');
        self.append(tag);
        self.append_chr('>');
    }

    fn close_tag(&mut self, tag: &str) {
        self.append("</");
        self.append(tag);
        self.append_chr('>');
    }

    fn void_tag(&mut self, tag: &str) {
        self.append_chr('<');
        self.append(tag);
        self.append("/>");
    }
}
