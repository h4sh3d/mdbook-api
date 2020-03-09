use crate::theme::Theme;

use std::collections::HashMap;

use mdbook::errors::Result;
use mdbook::renderer::RenderContext;
use mdbook::utils::fs::write_file;

static INDEX: &[u8] = include_bytes!("../../theme/index.hbs");
static FAVICON: &[u8] = include_bytes!("../../theme/favicon.png");
static APP_CSS: &[u8] = include_bytes!("../../theme/app.css");
static APP_JS: &[u8] = include_bytes!("../../theme/app.js");

#[derive(Debug)]
pub struct HtmlTheme {
    template: Vec<u8>,
    assets: HashMap<String, Vec<u8>>,
}

impl HtmlTheme {
    pub fn load_default_assests() -> HashMap<String, Vec<u8>> {
        let mut assets_map = HashMap::new();
        assets_map.insert("favicon.png".to_owned(), FAVICON.to_owned());
        assets_map.insert("app.css".to_owned(), APP_CSS.to_owned());
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
