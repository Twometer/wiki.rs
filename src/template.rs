use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub fn render_template(template: &str, variables: HashMap<&str, &str>) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\{\{([^}]*)\}\}"#).unwrap();
    }

    let mut output = template.to_owned();

    let captures: Vec<Captures> = RE.captures_iter(template).collect();
    for capture in captures.iter().rev() {
        let mustache = capture.get(0).unwrap();
        let name = capture.get(1).unwrap().as_str().trim().to_lowercase();

        if let Some(value) = variables.get(name.as_str()) {
            output.replace_range(mustache.range(), value);
        }
    }

    output
}
