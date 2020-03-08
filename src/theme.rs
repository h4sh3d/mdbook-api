//! Theme management for API documentation
//!
//! A theme is responsible for managing assests, in an HTML context those assests can
//! be css, javascript, images, template file, etc.
//!
//! Theme should provide a default setup and overloading methods to allow per project
//! customization.

// TODO add sass support
// TODO add theme extension support

pub static INDEX: &[u8] = include_bytes!("../theme/index.hbs");
pub static FAVICON: &[u8] = include_bytes!("../theme/favicon.png");
pub static APP_CSS: &[u8] = include_bytes!("../theme/css/app.css");
pub static APP_JS: &[u8] = include_bytes!("../theme/app.js");

#[derive(Debug)]
pub struct Theme {
    pub index: Vec<u8>,
    pub favicon: Vec<u8>,
    pub app_css: Vec<u8>,
    pub app_js: Vec<u8>,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            index: INDEX.to_owned(),
            favicon: FAVICON.to_owned(),
            app_css: APP_CSS.to_owned(),
            app_js: APP_JS.to_owned(),
        }
    }
}
