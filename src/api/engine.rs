use crate::engine::Engine;
use crate::api::parser::parser_from_str;

use pulldown_cmark::html;

use mdbook::book::BookItem;
use mdbook::errors::Result;
use mdbook::errors::ResultExt;
use mdbook::renderer::RenderContext;
use mdbook::utils;

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
        let config = &ctx.config;

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
