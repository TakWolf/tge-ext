use super::AssetHolder;
use tge::prelude::*;
use std::path::Path;

pub trait LoadableAsset {
    fn load_asset(engine: &mut Engine, path: impl AsRef<Path>) -> GameResult<AssetHolder>;
}

impl LoadableAsset for Texture {
    fn load_asset(engine: &mut Engine, path: impl AsRef<Path>) -> GameResult<AssetHolder> {
        Self::load(engine, path).map(|texture| texture.into())
    }
}

impl LoadableAsset for Font {
    fn load_asset(engine: &mut Engine, path: impl AsRef<Path>) -> GameResult<AssetHolder> {
        Self::load(engine, path).map(|font| font.into())
    }
}
