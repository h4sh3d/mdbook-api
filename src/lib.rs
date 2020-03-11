#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

pub mod api;
pub mod engine;
pub mod template;
pub mod theme;

pub use api::engine::HtmlEngine;
pub use api::template::{HtmlOnePageTemplate, HtmlTemplate};
pub use api::HtmlRenderer;

pub type ApiRenderer = HtmlRenderer<HtmlEngine, HtmlTemplate>;
pub type ApiOnePageRenderer = HtmlRenderer<HtmlEngine, HtmlOnePageTemplate>;
