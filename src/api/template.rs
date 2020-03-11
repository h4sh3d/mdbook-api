use crate::api::theme::HtmlTheme;
use crate::api::HtmlContext;
use crate::template::Template;
use crate::theme::Theme;

use handlebars::{Context, Handlebars, Helper, HelperDef, Output, RenderError};
use pulldown_cmark::{html, Event, Parser};
use regex::{Captures, Regex};
use serde::Serialize;
use std::collections::BTreeMap;
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
            handlebars.register_helper("toc", Box::new(RenderToc));

            let filepath = Path::new(&ch.path).with_extension("html");

            // Render the handlebars template with the data
            let rendered = handlebars.render("index", &input)?;
            let rendered = fix_code_blocks(&rendered);
            let rendered = fix_heading_ids(&rendered);

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
        handlebars.register_helper("toc", Box::new(RenderToc));

        // Render the handlebars template with the data
        let rendered = handlebars.render("index", &input)?;
        let rendered = fix_code_blocks(&rendered);
        let rendered = fix_heading_ids(&rendered);

        utils::fs::write_file(&ctx.destination, "index.html", rendered.as_bytes())
    }
}

pub fn fix_code_blocks(html: &str) -> String {
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

pub fn fix_heading_ids(html: &str) -> String {
    let regex = Regex::new(r##"<h([\d])>([^<]+)<"##).unwrap();
    regex
        .replace_all(html, |caps: &Captures<'_>| {
            let level = &caps[1];
            let title = &caps[2];
            let id = utils::normalize_id(&title);

            format!(
                r#"<h{level} id="{id}">{title}<"#,
                level = level,
                id = id,
                title = title,
            )
        })
        .into_owned()
}

// Handlebars helper to construct TOC
#[derive(Clone, Copy)]
pub struct RenderToc;

impl HelperDef for RenderToc {
    fn call<'reg: 'rc, 'rc>(
        &self,
        _h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars,
        ctx: &'rc Context,
        rc: &mut handlebars::RenderContext<'reg>,
        out: &mut dyn Output,
    ) -> std::result::Result<(), RenderError> {
        // get value from context data
        // rc.get_path() is current json parent path, you should always use it like this
        // param is the key of value you want to display
        let chapters = rc.evaluate(ctx, "@root/chapters").and_then(|c| {
            serde_json::value::from_value::<Vec<BTreeMap<String, String>>>(c.as_json().clone())
                .map_err(|_| RenderError::new("Could not decode the JSON data"))
        })?;

        out.write("<ul id=\"toc\" class=\"toc-list-h1\">")?;

        let mut current_level = 1;
        let mut close = false;

        for item in chapters {
            let (_, level) = if let Some(s) = item.get("section") {
                (s.as_str(), s.matches('.').count())
            } else {
                ("", 1)
            };

            if level > current_level {
                while level > current_level {
                    current_level += 1;
                    out.write(&format!("<ul class=\"toc-list-h{}\">", level))?;
                }
                out.write("<li>")?;
            } else if level < current_level {
                while level < current_level {
                    out.write("</ul>")?;
                    current_level -= 1;
                }
                out.write("<li>")?;
            } else {
                if close {
                    out.write("</li>")?;
                    close = false;
                }
                out.write("<li>")?;
            }

            if let Some(name) = item.get("name") {

                //let ancor = if let Some(path) = item.get("path") {
                //    if !path.is_empty() {
                //        let tmp = Path::new(path)
                //            .with_extension("");

                //        tmp.as_path()
                //            .file_name()
                //            .unwrap()
                //            .to_str()
                //            .unwrap()
                //            .to_owned()
                //    } else {
                //        "".to_owned()
                //    }
                //} else {
                //    "".to_owned()
                //};

                out.write(&format!("<a href=\"#{}\" ", utils::normalize_id(name)))?;
                out.write(&format!("class=\"toc-h{} toc-link\" ", level))?;

                // Render only inline code blocks

                // filter all events that are not inline code blocks
                let parser = Parser::new(name).filter(|event| match *event {
                    Event::Code(_) | Event::Html(_) | Event::Text(_) => true,
                    _ => false,
                });

                // render markdown to html
                let mut markdown_parsed_name = String::new();
                html::push_html(&mut markdown_parsed_name, parser);

                // write to the handlebars template
                out.write(&format!("data-title=\"{}\">", name))?;
                out.write(&markdown_parsed_name)?;

                out.write("</a>")?;
                close = true;
            }
        }

        while current_level > 1 {
            if close {
                out.write("</li>")?;
                close = false;
            }

            out.write("</ul>")?;
            out.write("</li>")?;
            current_level -= 1;
        }

        out.write("</ul>")?;
        Ok(())
    }
}
