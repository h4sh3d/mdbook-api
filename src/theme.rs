//! Theme management for HTML documentation
//!
//! A theme is responsible for managing assests, in an HTML context those assests can
//! be css, javascript, images, template file, etc.
//!
//! Theme should provide a default setup and overloading methods to allow per project
//! customization.

use mdbook::renderer::{RenderContext};
use mdbook::errors::Result;
use mdbook::utils::fs::write_file;
use std::collections::HashMap;

// TODO add sass support
// TODO add theme extension support
// TODO add font Awsome support

static INDEX: &[u8] = include_bytes!("../theme/index.hbs");
static FAVICON: &[u8] = include_bytes!("../theme/favicon.png");
static APP_CSS: &[u8] = include_bytes!("../theme/css/app.css");
static APP_JS: &[u8] = include_bytes!("../theme/app.js");

pub trait Theme: Sized {
    fn load_from_context(ctx: &RenderContext) -> Result<Self>;

    fn copy_static_files(&self, ctx: &RenderContext) -> Result<()>;

    fn get_template(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct HtmlTheme {
    template: Vec<u8>,
    assets: HashMap<String, Vec<u8>>,
}

impl HtmlTheme {
    pub fn load_default_assests() -> HashMap<String, Vec<u8>> {
        let mut assets_map = HashMap::new();
        assets_map.insert("favicon.png".to_owned(), FAVICON.to_owned());
        assets_map.insert("css/app.css".to_owned(), APP_CSS.to_owned());
        assets_map.insert("app.js".to_owned(), APP_JS.to_owned());
        assets_map
    }
}

impl Theme for HtmlTheme {
    /// Load a HTML theme from a render context
    fn load_from_context(_ctx: &RenderContext) -> Result<Self> {
        let assets = Self::load_default_assests();

        // TODO overload assets if present in context
        // TODO add assets present in theme folder, like imgs

        Ok(HtmlTheme {
            template: INDEX.into(),
            assets,
        })
    }

    fn copy_static_files(&self, ctx: &RenderContext) -> Result<()> {
        let destination = &ctx.destination;

        write_file(
            destination,
            ".nojekyll",
            b"This file makes sure that Github Pages doesn't process mdBook's output.",
        )?;

        for (name, content) in &self.assets {
            write_file(destination, name, &content)?;
        }

        Ok(())
    }

    fn get_template(&self) -> Vec<u8> {
        self.template.clone()
    }
}
