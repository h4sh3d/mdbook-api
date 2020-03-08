//! HTML Basic renderer
//!
//! An HTML renderer is a basic, extensible Markdown to HTML renderer
//! engine for `mdbook`.

use crate::template::HtmlTemplate;

use mdbook::renderer::{RenderContext, Renderer};

/// Must be extensible and configurable but shoud implement the logic
/// for rendering HTML project, combining theme and template management.
#[derive(Default)]
pub struct HtmlRenderer<T: HtmlEngine + Default> {
    engine: T,
}

impl<T: HtmlEngine + Default> HtmlRenderer<T> {
    fn name(&self) -> &str {
        self.engine.name()
    }

    fn render(&self, ctx: &RenderContext) -> mdbook::errors::Result<()> {

        // TODO Theme management
        // TODO Template management

        let template = HtmlTemplate::new();
        template.render(&ctx)
    }
}

pub trait HtmlEngine {
    fn name(&self) -> &str;
}

/// Implement mdbook `Renderer` for all HtmlRenderer
impl<T> Renderer for HtmlRenderer<T> where T: HtmlEngine + Default  {
    fn name(&self) -> &str {
        self.name()
    }

    fn render(&self, ctx: &RenderContext) -> mdbook::errors::Result<()> {
        self.render(&ctx)
    }
}
