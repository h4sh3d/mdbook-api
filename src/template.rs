//! HTML template management
//!
//! A template should provide methods to output HTML from markdown files.
//!
//! Two part are required:
//!
//!  * Managing the page template
//!  * Managing the content output from Markdown

use std::path::{Path};
use handlebars::Handlebars;
use regex::{Captures, Regex};
use serde::Serialize;

use mdbook::book::{BookItem};
use mdbook::renderer::RenderContext;
use mdbook::utils;
use mdbook::errors::Result;

use crate::theme::Theme;
use crate::theme::HtmlTheme;

pub trait Template<I: Serialize>: Sized {
    type Theme: Theme;

    fn load_from_context(ctx: &RenderContext) -> Result<Self>;

    fn render_chapter(&self, ctx: &RenderContext, theme: &Self::Theme, item: &BookItem, input: &mut I) -> Result<()>;
}

#[derive(Debug, Default)]
pub struct HtmlTemplate;

impl<I> Template<I> for HtmlTemplate where I: Serialize {
    type Theme = HtmlTheme;

    fn load_from_context(_ctx: &RenderContext) -> Result<Self> {
        Ok(HtmlTemplate)
    }

    fn render_chapter(&self, ctx: &RenderContext, theme: &Self::Theme, item: &BookItem, input: &mut I) -> Result<()> {

        if let BookItem::Chapter(ref ch) = *item {
            let mut handlebars = Handlebars::new();

            handlebars.register_template_string("index", String::from_utf8(theme.get_template())?)?;

            // TODO add partial handlebars template
            //debug!("Register the header handlebars template");
            //handlebars.register_partial("header", String::from_utf8(theme.header.clone())?)?;

            // TODO add helpers hook
            //debug!("Register handlebars helpers");
            //self.register_hbs_helpers(&mut handlebars, &html_config);
            //handlebars.register_helper(
            //    "toc",
            //    Box::new(helpers::toc::RenderToc {
            //        no_section_label: html_config.no_section_label,
            //    }),
            //);

            let filepath = Path::new(&ch.path).with_extension("html");

            // Print version
            //let mut print_content = String::new();

            //let mut is_index = true;


            // Render the handlebars template with the data
            let rendered = handlebars.render("index", &input)?;
            // TODO fixe html stream
            let rendered = fix_code_blocks(&rendered);

            //let rendered = self.post_process(rendered, &ctx.html_config.playpen);

            // Write to file
            utils::fs::write_file(&ctx.destination, &filepath, rendered.as_bytes())?;

        }

        Ok(())
    }

}

fn fix_code_blocks(html: &str) -> String {
    let regex = Regex::new(r##"<pre><code([^>]+)class="([^"]+)"([^>]*)>"##).unwrap();
    regex
        .replace_all(html, |caps: &Captures<'_>| {
            let before = &caps[1];
            let classes = &caps[2].replace(",", " ");
            let after = &caps[3];

            let inner_regex = Regex::new(r##"language-([^"]+)"##).unwrap();
            let classes = inner_regex
                .replace_all(classes, |inner_caps: &Captures<'_>| {
                    let language: Vec<&str> = inner_caps[1].split(' ').collect();

                    format!(
                        r#"language-{lang} tab-{lang} highlight"#,
                        lang = language[0],
                    )
                })
                .into_owned();

            format!(
                r#"<pre class="{classes}"><code{before}{after}>"#,
                before = before,
                classes = classes,
                after = after
            )
        })
        .into_owned()
}
