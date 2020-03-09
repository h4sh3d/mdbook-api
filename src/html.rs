//! HTML Basic renderer
//!
//! An HTML renderer is a basic, extensible Markdown to HTML renderer
//! engine for `mdbook`.

use crate::template::Template;
use crate::theme::Theme;

use std::fs;
use serde::Serialize;

use mdbook::utils;
use mdbook::errors::ResultExt;
use mdbook::book::{BookItem};
use mdbook::renderer::{RenderContext, Renderer};
use mdbook::errors::Result;

pub struct HtmlContext {
    pub book_item: BookItem,
    pub is_index: bool,
}

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

        let mut is_index = true;
        for item in book.iter() {
            let html_item = HtmlContext {
                book_item: item.clone(),
                is_index,
            };

            let mut data = self.engine.process_chapter(&ctx, &html_item)?;

            self.template.render_chapter(&ctx, &self.theme, &html_item, &mut data)?;
            is_index = false;
        }

        self.theme.copy_static_files(&ctx)
    }
}

pub trait Engine<C>: Sized {
    type Output: Serialize;

    fn name(&self) -> &str;

    fn load_from_context(ctx: &RenderContext) -> Result<Self>;

    fn process_chapter(&self, ctx: &RenderContext, item: &C) -> Result<Self::Output>;
}

/// Implement mdbook `Renderer` for all HtmlRenderer
impl<E, T> Renderer for HtmlRenderer<E, T> where E: Engine<HtmlContext>, T: Template<HtmlContext, E::Output>  {
    fn name(&self) -> &str {
        self.name()
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        self.render(&ctx)
    }
}
