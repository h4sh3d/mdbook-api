//! HTML template management
//!
//! A template should provide methods to output HTML from markdown files.
//!
//! Two part are required:
//!
//!  * Managing the page template
//!  * Managing the content output from Markdown

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use handlebars::Handlebars;
use regex::{Captures, Regex};

use mdbook::book::{Book, BookItem};
use mdbook::config::{Config, HtmlConfig};
use mdbook::errors::ResultExt;
use mdbook::renderer::RenderContext;
use mdbook::utils;

use crate::theme::Theme;

#[derive(Default)]
pub struct HtmlTemplate;

impl HtmlTemplate {
    pub fn new() -> Self {
        HtmlTemplate
    }

    pub fn render(&self, ctx: &RenderContext) -> mdbook::errors::Result<()> {
        let html_config = ctx.config.html_config().unwrap_or_default();
        let destination = &ctx.destination;
        let book = &ctx.book;

        if destination.exists() {
            utils::fs::remove_dir_content(destination)
                .chain_err(|| "Unable to remove stale HTML output")?;
        }

        trace!("render");
        let mut handlebars = Handlebars::new();

        //let theme = theme::Theme::new(theme_dir);
        let theme = Theme::default();

        debug!("Register the index handlebars template");
        handlebars.register_template_string("index", String::from_utf8(theme.index.clone())?)?;

        //debug!("Register the header handlebars template");
        //handlebars.register_partial("header", String::from_utf8(theme.header.clone())?)?;

        //debug!("Register handlebars helpers");
        //self.register_hbs_helpers(&mut handlebars, &html_config);
        //handlebars.register_helper(
        //    "toc",
        //    Box::new(helpers::toc::RenderToc {
        //        no_section_label: html_config.no_section_label,
        //    }),
        //);

        let data = make_data(&book, &ctx.config, &html_config)?;

        // Print version
        let mut print_content = String::new();

        fs::create_dir_all(&destination)
            .chain_err(|| "Unexpected error when constructing destination path")?;

        let mut is_index = true;
        for item in book.iter() {
            let ctx = HtmlItem {
                handlebars: &handlebars,
                destination: destination.to_path_buf(),
                data: data.clone(),
                is_index,
                html_config: html_config.clone(),
            };
            self.render_item(item, ctx, &mut print_content)?;
            is_index = false;
        }

        //// Print version
        //self.configure_print_version(&mut data, &print_content);
        //if let Some(ref title) = ctx.config.book.title {
        //    data.insert("title".to_owned(), json!(title));
        //}

        //// Render the handlebars template with the data
        //debug!("Render template");
        //let rendered = handlebars.render("index", &data)?;

        //let rendered = self.post_process(rendered, &html_config.playpen);

        //utils::fs::write_file(&destination, "print.html", rendered.as_bytes())?;
        //debug!("Creating print.html ✓");

        debug!("Copy static files");
        self.copy_static_files(&destination, &theme)
            .chain_err(|| "Unable to copy across static files")?;

        //// Copy all remaining files
        //utils::fs::copy_files_except_ext(&src_dir, &destination, true, &["md"])?;

        Ok(())
    }

    fn render_item(
        &self,
        item: &BookItem,
        mut ctx: HtmlItem<'_>,
        print_content: &mut String,
    ) -> mdbook::errors::Result<()> {
        if let BookItem::Chapter(ref ch) = *item {
            let content = ch.content.clone();
            let content = utils::render_markdown(&content, ctx.html_config.curly_quotes);

            let fixed_content = utils::render_markdown_with_path(
                &ch.content,
                ctx.html_config.curly_quotes,
                Some(&ch.path),
            );
            print_content.push_str(&fixed_content);

            // Update the context with data for this file
            let path = ch
                .path
                .to_str()
                .chain_err(|| "Could not convert path to str")?;
            let filepath = Path::new(&ch.path).with_extension("html");

            // Non-lexical lifetimes needed :'(
            let title: String;
            {
                let book_title = ctx
                    .data
                    .get("book_title")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("");

                title = match book_title {
                    "" => ch.name.clone(),
                    _ => ch.name.clone() + " - " + book_title,
                }
            }

            ctx.data.insert("path".to_owned(), json!(path));
            ctx.data.insert("content".to_owned(), json!(content));
            ctx.data.insert("chapter_title".to_owned(), json!(ch.name));
            ctx.data.insert("title".to_owned(), json!(title));
            ctx.data.insert(
                "path_to_root".to_owned(),
                json!(utils::fs::path_to_root(&ch.path)),
            );
            if let Some(ref section) = ch.number {
                ctx.data
                    .insert("section".to_owned(), json!(section.to_string()));
            }

            // Render the handlebars template with the data
            debug!("Render template");
            let rendered = ctx.handlebars.render("index", &ctx.data)?;
            let rendered = fix_code_blocks(&rendered);

            //let rendered = self.post_process(rendered, &ctx.html_config.playpen);

            // Write to file
            debug!("Creating {}", filepath.display());
            utils::fs::write_file(&ctx.destination, &filepath, rendered.as_bytes())?;

            if ctx.is_index {
                ctx.data.insert("path".to_owned(), json!("index.md"));
                ctx.data.insert("path_to_root".to_owned(), json!(""));
                ctx.data.insert("is_index".to_owned(), json!("true"));
                let rendered_index = ctx.handlebars.render("index", &ctx.data)?;
                let rendered_index = fix_code_blocks(&rendered_index);
                debug!("Creating index.html from {}", path);
                utils::fs::write_file(&ctx.destination, "index.html", rendered_index.as_bytes())?;
            }
        }

        Ok(())
    }

    fn copy_static_files(
        &self,
        destination: &Path,
        theme: &Theme,
    ) -> mdbook::errors::Result<()> {
        use mdbook::utils::fs::write_file;

        write_file(
            destination,
            ".nojekyll",
            b"This file makes sure that Github Pages doesn't process mdBook's output.",
        )?;

        write_file(destination, "favicon.png", &theme.favicon)?;

        write_file(destination, "app.js", &theme.app_js)?;
        write_file(destination, "css/app.css", &theme.app_css)?;

        Ok(())
    }
}

pub struct HtmlItem<'a> {
    handlebars: &'a Handlebars,
    destination: PathBuf,
    data: serde_json::Map<String, serde_json::Value>,
    is_index: bool,
    html_config: HtmlConfig,
}

fn make_data(
    book: &Book,
    config: &Config,
    html_config: &HtmlConfig,
) -> mdbook::errors::Result<serde_json::Map<String, serde_json::Value>> {
    trace!("make_data");

    let mut data = serde_json::Map::new();
    data.insert(
        "language".to_owned(),
        json!(config.book.language.clone().unwrap_or_default()),
    );
    data.insert(
        "book_title".to_owned(),
        json!(config.book.title.clone().unwrap_or_default()),
    );
    data.insert(
        "description".to_owned(),
        json!(config.book.description.clone().unwrap_or_default()),
    );
    data.insert("favicon".to_owned(), json!("favicon.png"));
    if let Some(ref livereload) = html_config.livereload_url {
        data.insert("livereload".to_owned(), json!(livereload));
    }

    // Add google analytics tag
    if let Some(ref ga) = html_config.google_analytics {
        data.insert("google_analytics".to_owned(), json!(ga));
    }

    let search = html_config.search.clone();
    if cfg!(feature = "search") {
        let search = search.unwrap_or_default();
        data.insert("search_enabled".to_owned(), json!(search.enable));
        data.insert(
            "search_js".to_owned(),
            json!(search.enable && search.copy_js),
        );
    } else if search.is_some() {
        warn!("mdBook compiled without search support, ignoring `output.html.search` table");
        warn!(
            "please reinstall with `cargo install mdbook --force --features search`to use the \
             search feature"
        )
    }

    let mut chapters = vec![];

    for item in book.iter() {
        // Create the data to inject in the template
        let mut chapter = BTreeMap::new();

        match *item {
            BookItem::Chapter(ref ch) => {
                if let Some(ref section) = ch.number {
                    chapter.insert("section".to_owned(), json!(section.to_string()));
                }

                chapter.insert(
                    "has_sub_items".to_owned(),
                    json!((!ch.sub_items.is_empty()).to_string()),
                );

                chapter.insert("name".to_owned(), json!(ch.name));
                let path = ch
                    .path
                    .to_str()
                    .chain_err(|| "Could not convert path to str")?;
                chapter.insert("path".to_owned(), json!(path));
            }
            BookItem::Separator => {
                chapter.insert("spacer".to_owned(), json!("_spacer_"));
            }
        }

        chapters.push(chapter);
    }

    data.insert("chapters".to_owned(), json!(chapters));

    debug!("[*]: JSON constructed");
    Ok(data)
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
