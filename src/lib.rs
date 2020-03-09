#[macro_use]
extern crate serde_json;

pub mod theme;
pub mod template;
pub mod html;

use std::collections::BTreeMap;

use crate::html::{HtmlRenderer, HtmlContext, Engine};
use crate::template::HtmlTemplate;

use mdbook::utils;
use mdbook::book::{BookItem};
use mdbook::errors::ResultExt;
use mdbook::renderer::RenderContext;
use mdbook::errors::Result;

pub struct ApiEngine {
    data: serde_json::Map<String, serde_json::Value>,
}

impl Engine<HtmlContext> for ApiEngine {
    type Output = serde_json::Map<String, serde_json::Value>;

    fn name(&self) -> &str {
        "api"
    }

    fn load_from_context(ctx: &RenderContext) -> Result<Self> {
        let config = &ctx.config;
        let book = &ctx.book;

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
        Ok(ApiEngine { data })
    }

    fn process_chapter(&self, _ctx: &RenderContext, item: &HtmlContext) -> Result<Self::Output> {
        // Clone the base data and apply changes based on chapter
        let mut data = self.data.clone();

        if let BookItem::Chapter(ref ch) = &item.book_item {
            // Update the context with data for this file
            let path = ch
                .path
                .to_str()
                .chain_err(|| "Could not convert path to str")?;

            // Non-lexical lifetimes needed :'(
            let title: String;
            {
                let book_title = data
                    .get("book_title")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("");

                title = match book_title {
                    "" => ch.name.clone(),
                    _ => ch.name.clone() + " - " + book_title,
                }
            }

            data.insert("path".to_owned(), json!(path));

            let content = utils::render_markdown(&ch.content, false);
            data.insert("content".to_owned(), json!(content));

            data.insert("chapter_title".to_owned(), json!(ch.name));
            data.insert("title".to_owned(), json!(title));
            data.insert(
                "path_to_root".to_owned(),
                json!(utils::fs::path_to_root(&ch.path)),
            );
            if let Some(ref section) = ch.number {
                data.insert("section".to_owned(), json!(section.to_string()));
            }

            if item.is_index {
                data.insert("path".to_owned(), json!("index.md"));
                data.insert("path_to_root".to_owned(), json!(""));
                data.insert("is_index".to_owned(), json!("true"));
            }
        }

        Ok(data)
    }
}

pub type ApiRenderer = HtmlRenderer<ApiEngine, HtmlTemplate>;
