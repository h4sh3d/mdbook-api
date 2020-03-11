//! Engine is responsible for processing data and returning the a serialized
//! version reading for template

use serde::Serialize;

use mdbook::errors::Result;
use mdbook::renderer::RenderContext;

pub trait Engine<C>: Sized {
    type Output: Serialize;

    fn name(&self) -> &str;

    fn load_from_context(ctx: &RenderContext) -> Result<Self>;

    fn process_chapter(&self, ctx: &RenderContext, item: &mut C) -> Result<Self::Output>;

    fn finalize_book(&self, ctx: &RenderContext, item: &mut C) -> Result<Self::Output>;
}
