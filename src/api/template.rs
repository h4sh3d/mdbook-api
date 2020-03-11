use crate::api::theme::HtmlTheme;
use crate::api::HtmlContext;
use crate::template::Template;
use crate::theme::Theme;

use handlebars::Handlebars;
use regex::{Captures, Regex};
use serde::Serialize;
use std::path::Path;

use mdbook::book::BookItem;
use mdbook::errors::Result;
use mdbook::renderer::RenderContext;
use mdbook::utils;

#[derive(Debug, Default)]
pub struct HtmlTemplate;

impl<I> Template<HtmlContext, I> for HtmlTemplate
where
    I: Serialize,
{
    type Theme = HtmlTheme;

    fn load_from_context(_ctx: &RenderContext) -> Result<Self> {
        Ok(HtmlTemplate)
    }

    fn render_chapter(
        &self,
        ctx: &RenderContext,
        theme: &Self::Theme,
        item: &mut HtmlContext,
        input: &mut I,
    ) -> Result<()> {
        if let Some(BookItem::Chapter(ref ch)) = &item.book_item {
            let mut handlebars = Handlebars::new();

            handlebars
                .register_template_string("index", String::from_utf8(theme.get_template())?)?;

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

            // Render the handlebars template with the data
            let rendered = handlebars.render("index", &input)?;
            // TODO fixe html stream
            let rendered = fix_code_blocks(&rendered);

            // Write to file
            if item.is_index {
                utils::fs::write_file(&ctx.destination, "index.html", rendered.as_bytes())?;
            } else {
                utils::fs::write_file(&ctx.destination, &filepath, rendered.as_bytes())?;
            }
        }

        Ok(())
    }
}

/// One pager html template
#[derive(Debug, Default)]
pub struct HtmlOnePageTemplate;

impl<I> Template<HtmlContext, I> for HtmlOnePageTemplate
where
    I: Serialize,
{
    type Theme = HtmlTheme;

    fn load_from_context(_ctx: &RenderContext) -> Result<Self> {
        Ok(HtmlOnePageTemplate)
    }

    fn finalize_book(&self, ctx: &RenderContext, theme: &Self::Theme, input: &mut I) -> Result<()> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("index", String::from_utf8(theme.get_template())?)?;

        // Render the handlebars template with the data
        let rendered = handlebars.render("index", &input)?;
        let rendered = fix_code_blocks(&rendered);
        utils::fs::write_file(&ctx.destination, "index.html", rendered.as_bytes())
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
