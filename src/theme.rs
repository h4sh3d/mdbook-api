//! A theme is responsible for managing assests, in an HTML context those assests can
//! be css, javascript, images, template file, etc. or simply images.
//!
//! Theme should provide a default setup and overloading methods to allow per project
//! customization.

use mdbook::errors::Result;
use mdbook::renderer::RenderContext;

// TODO add sass support
// TODO add theme extension support
// TODO add font Awsome support

pub trait Theme: Sized {
    fn load_from_context(ctx: &RenderContext) -> Result<Self>;

    fn copy_static_files(&self, ctx: &RenderContext) -> Result<()>;

    fn get_template(&self) -> Vec<u8>;
}
