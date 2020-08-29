mod sprite;
mod frame;
mod animation;
mod resolution;

pub use sprite::Sprite;
pub use frame::Frame;
pub use animation::Animation;
pub use resolution::{ResolutionPolicy, ResolutionAdapter, CanvasResolutionAdapter, TransformResolutionAdapter};

use crate::asset::TextureRefProvider;
use tge::prelude::*;

fn get_texture_region(provider: &impl TextureRefProvider, res_name: impl AsRef<str>) -> GameResult<Region> {
    let texture_size = provider.texture_ref(res_name)?
        .texture_size()
        .ok_or_else(|| GameError::RuntimeError("no texture".into()))?;
    Ok(Region::new(0.0, 0.0, texture_size.width as f32, texture_size.height as f32))
}
