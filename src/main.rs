use mdbook_api::HtmlHandlebars;

use mdbook::renderer::{RenderContext, Renderer};
use std::io;

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    let renderer = HtmlHandlebars::new();
    renderer.render(&ctx).expect("Failed to render");
}
