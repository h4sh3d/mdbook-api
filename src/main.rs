use mdbook_api::ApiRenderer;

use mdbook::renderer::RenderContext;
use std::io;

fn main() {
    // Get the mdbook context from stdin
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    // Render the API documentation with the ApiRenderer
    let renderer = ApiRenderer::new(&ctx).expect("Failed to load renderer");
    renderer.render(&ctx).expect("Failed to render");
}
