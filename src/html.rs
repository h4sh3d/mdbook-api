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

/// Must be extensible and configurable but shoud implement the logic
/// for rendering HTML project, combining theme and template management.
#[derive(Default)]
pub struct HtmlRenderer<E: Engine, T: Template<E::Output>> {
    engine: E,
    template: T,
    theme: T::Theme,
}

impl<E: Engine, T: Template<E::Output>> HtmlRenderer<E, T> {
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

        for item in book.iter() {
            //if ctx.is_index {
            //    ctx.data.insert("path".to_owned(), json!("index.md"));
            //    ctx.data.insert("path_to_root".to_owned(), json!(""));
            //    ctx.data.insert("is_index".to_owned(), json!("true"));
            //    let rendered_index = ctx.handlebars.render("index", &ctx.data)?;
            //    let rendered_index = fix_code_blocks(&rendered_index);
            //    debug!("Creating index.html from {}", path);
            //    utils::fs::write_file(&ctx.destination, "index.html", rendered_index.as_bytes())?;
            //}
            let mut data = self.engine.process_chapter(&item, &ctx)?;

            self.template.render_chapter(&ctx, &self.theme, &item, &mut data)?;
        }

        self.theme.copy_static_files(&ctx)
    }
}

pub trait Engine: Sized {
    type Output: Serialize;

    fn name(&self) -> &str;

    fn load_from_context(ctx: &RenderContext) -> Result<Self>;

    fn process_chapter(&self, item: &BookItem, ctx: &RenderContext) -> Result<Self::Output>;
}

/// Implement mdbook `Renderer` for all HtmlRenderer
impl<E, T> Renderer for HtmlRenderer<E, T> where E: Engine, T: Template<E::Output>  {
    fn name(&self) -> &str {
        self.name()
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        self.render(&ctx)
    }
}
