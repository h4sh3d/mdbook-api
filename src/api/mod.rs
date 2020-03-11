//! HTML API renderer
//!
//! An HTML renderer is a basic, extensible Markdown to HTML renderer
//! engine for `mdbook`.

use crate::engine::Engine;
use crate::template::Template;
use crate::theme::Theme;

use std::fs;

use mdbook::errors::Result;
use mdbook::errors::ResultExt;
use mdbook::renderer::{RenderContext, Renderer};
use mdbook::utils;

pub mod engine;
pub mod template;
pub mod theme;

use engine::HtmlContext;

/// Must be extensible and configurable but shoud implement the logic
/// for rendering HTML project, combining theme and template management.
#[derive(Default)]
pub struct HtmlRenderer<E: Engine<HtmlContext>, T: Template<HtmlContext, E::Output>> {
    engine: E,
    template: T,
    theme: T::Theme,
}

impl<E: Engine<HtmlContext>, T: Template<HtmlContext, E::Output>> HtmlRenderer<E, T> {
    pub fn new(ctx: &RenderContext) -> Result<Self> {
        Ok(HtmlRenderer {
            engine: E::load_from_context(&ctx)?,
            template: T::load_from_context(&ctx)?,
            theme: T::Theme::load_from_context(&ctx)?,
        })
    }

    pub fn name(&self) -> &str {
        self.engine.name()
    }

    fn clean_dest(&self, ctx: &RenderContext) -> Result<()> {
        if ctx.destination.exists() {
            utils::fs::remove_dir_content(&ctx.destination)
                .chain_err(|| "Unable to remove stale HTML output")?;
        }
        Ok(())
    }

    pub fn render(&self, ctx: &RenderContext) -> Result<()> {
        let book = &ctx.book;
        let destination = &ctx.destination;

        self.clean_dest(&ctx)?;

        fs::create_dir_all(&destination)
            .chain_err(|| "Unexpected error when constructing destination path")?;

        self.template.initialize_book(&ctx, &self.theme)?;

        let mut html_ctx = HtmlContext::default();
        html_ctx.is_index = true;

        for item in book.iter() {
            html_ctx.book_item = Some(item.clone());

            let mut data = self.engine.process_chapter(&ctx, &mut html_ctx)?;

            self.template
                .render_chapter(&ctx, &self.theme, &mut html_ctx, &mut data)?;

            html_ctx.is_index = false;
        }

        let mut data = self.engine.finalize_book(&ctx, &mut html_ctx)?;
        self.template.finalize_book(&ctx, &self.theme, &mut data)?;

        self.theme.copy_static_files(&ctx)
    }
}

/// Implement mdbook `Renderer` for all HtmlRenderer
impl<E, T> Renderer for HtmlRenderer<E, T>
where
    E: Engine<HtmlContext>,
    T: Template<HtmlContext, E::Output>,
{
    fn name(&self) -> &str {
        self.name()
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        self.render(&ctx)
    }
}
