use crate::api::engine::ApiConfig;
use crate::theme::Theme;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use mdbook::errors::Result;
use mdbook::renderer::RenderContext;
use mdbook::utils::fs::write_file;

static INDEX: &[u8] = include_bytes!("../../theme/index.hbs");
static FAVICON: &[u8] = include_bytes!("../../theme/favicon.png");
static APP_CSS: &[u8] = include_bytes!("../../theme/app.css");
static APP_JS: &[u8] = include_bytes!("../../theme/app.js");
static LOGO: &[u8] = include_bytes!("../../theme/logo.png");
static NAVBAR: &[u8] = include_bytes!("../../theme/navbar.png");

static FONT_EOT: &[u8] = include_bytes!("../../theme/fonts/slate.eot");
static FONT_SVG: &[u8] = include_bytes!("../../theme/fonts/slate.svg");
static FONT_TTF: &[u8] = include_bytes!("../../theme/fonts/slate.ttf");
static FONT_WOFF: &[u8] = include_bytes!("../../theme/fonts/slate.woff");
static FONT_WOFF2: &[u8] = include_bytes!("../../theme/fonts/slate.woff2");

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
        assets_map.insert("logo.png".to_owned(), LOGO.to_owned());
        assets_map.insert("navbar.png".to_owned(), NAVBAR.to_owned());

        assets_map.insert("fonts/slate.eot".to_owned(), FONT_EOT.to_owned());
        assets_map.insert("fonts/slate.svg".to_owned(), FONT_SVG.to_owned());
        assets_map.insert("fonts/slate.ttf".to_owned(), FONT_TTF.to_owned());
        assets_map.insert("fonts/slate.woff".to_owned(), FONT_WOFF.to_owned());
        assets_map.insert("fonts/slate.woff2".to_owned(), FONT_WOFF2.to_owned());
        assets_map
    }
}

// TODO add assets present in theme folder, like imgs

impl Theme for HtmlTheme {
    /// Load a HTML theme from a render context
    fn load_from_context(ctx: &RenderContext) -> Result<Self> {
        let config = &ctx.config;
        let api_config: ApiConfig = match config.get_deserialized_opt("output.api") {
            Ok(Some(config)) => Some(config),
            _ => None,
        }
        .unwrap_or_default();

        let mut assets = Self::load_default_assests();
        let mut template = INDEX.to_owned();

        let theme_dir = if let Some(path) = api_config.theme_dir {
            ctx.root.join(Path::new(&path).to_path_buf())
        } else {
            ctx.root.join("theme")
        };

        if theme_dir.exists() && theme_dir.is_dir() {
            // Overload assets if present in theme_dir
            for (name, mut content) in &mut assets {
                let filename = theme_dir.join(name);

                if !filename.exists() {
                    continue;
                }

                load_file_contents(&filename, &mut content)?;
            }

            // Overload index template
            let filename = theme_dir.join("index.hbs");

            if filename.exists() {
                load_file_contents(&filename, &mut template)?;
            }
        }

        Ok(HtmlTheme { template, assets })
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

/// Checks if a file exists, if so, the destination buffer will be filled with
/// its contents.
pub fn load_file_contents<P: AsRef<Path>>(filename: P, dest: &mut Vec<u8>) -> Result<()> {
    let filename = filename.as_ref();

    let mut buffer = Vec::new();
    File::open(filename)?.read_to_end(&mut buffer)?;

    // We needed the buffer so we'd only overwrite the existing content if we
    // could successfully load the file into memory.
    dest.clear();
    dest.append(&mut buffer);

    Ok(())
}
