use crate::api::parser::parser_from_str;
use crate::engine::Engine;

use pulldown_cmark::html;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use mdbook::book::BookItem;
use mdbook::errors::Result;
use mdbook::errors::ResultExt;
use mdbook::renderer::RenderContext;
use mdbook::utils;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ApiConfig {
    pub theme_dir: Option<String>,
    pub lang: Vec<Language>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Language {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LangLink {
    pub id: String,
    pub name: String,
}

#[derive(Default)]
pub struct HtmlContext {
    // Current book item
    pub book_item: Option<BookItem>,
    // If the book item is the first one
    pub is_index: bool,
    // Accumulate the content of all book items
    pub full_content: String,
}

// Prepare data for HTML rendering with Handlebar
pub struct HtmlEngine {
    data: serde_json::Map<String, serde_json::Value>,
}

impl Engine<HtmlContext> for HtmlEngine {
    type Output = serde_json::Map<String, serde_json::Value>;

    fn name(&self) -> &str {
        "api"
    }

    fn load_from_context(ctx: &RenderContext) -> Result<Self> {
        let book = &ctx.book;
        let config = &ctx.config;

        let html_config = &ctx.config.html_config().unwrap_or_default();

        let api_config: ApiConfig = match config.get_deserialized_opt("output.api") {
            Ok(Some(config)) => Some(config),
            _ => None,
        }
        .unwrap_or_default();

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

        let mut lang_list = vec![];
        let mut languages = vec![];

        for lang in api_config.lang {
            lang_list.push(lang.id.clone());

            let l = if let Some(name) = lang.name {
                LangLink {
                    id: lang.id.clone(),
                    name,
                }
            } else {
                LangLink {
                    id: lang.id.clone(),
                    name: lang.id.clone(),
                }
            };
            languages.push(l);
        }

        data.insert(
            "lang_list".to_owned(),
            json!(serde_json::to_string(&lang_list)?),
        );
        data.insert("languages".to_owned(), json!(&languages));

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

        Ok(HtmlEngine { data })
    }

    fn process_chapter(
        &self,
        _ctx: &RenderContext,
        item: &mut HtmlContext,
    ) -> Result<Self::Output> {
        // Clone the base data and apply changes based on chapter
        let mut data = self.data.clone();

        if let Some(BookItem::Chapter(ref ch)) = &item.book_item {
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

            let mut content = String::new();
            let events = parser_from_str(&ch.content);
            html::push_html(&mut content, events);

            item.full_content.push_str(&content);
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

    fn finalize_book(&self, _ctx: &RenderContext, item: &mut HtmlContext) -> Result<Self::Output> {
        let mut data = self.data.clone();
        data.insert("content".to_owned(), json!(item.full_content));
        data.insert("path".to_owned(), json!("index.md"));
        data.insert("path_to_root".to_owned(), json!(""));
        data.insert("is_index".to_owned(), json!("true"));
        Ok(data)
    }
}
