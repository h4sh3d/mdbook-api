//! A template should provide methods to render a chapter given a theme to apply.
//! The chapter is rendered with data provided by an engine.

use serde::Serialize;

use mdbook::errors::Result;
use mdbook::renderer::RenderContext;

use crate::theme::Theme;

pub trait Template<C, I: Serialize>: Sized {
    type Theme: Theme;

    fn load_from_context(ctx: &RenderContext) -> Result<Self>;

    fn render_chapter(
        &self,
        ctx: &RenderContext,
        theme: &Self::Theme,
        item: &C,
        input: &mut I,
    ) -> Result<()>;
}
