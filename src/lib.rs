#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

pub mod theme;
pub mod template;
pub mod html;

use html::{HtmlRenderer, HtmlEngine};

#[derive(Default)]
pub struct ApiEngine;

impl HtmlEngine for ApiEngine {
    fn name(&self) -> &str {
        "api"
    }
}

pub type ApiRenderer = HtmlRenderer<ApiEngine>;
