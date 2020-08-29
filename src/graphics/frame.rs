use crate::asset::TextureRefProvider;
use tge::prelude::*;

#[derive(Clone)]
pub struct Frame {
    pub(crate) res_name: String,
    pub(crate) region: Region,
    pub(crate) origin: Position,
}

impl Frame {
    pub fn new(res_name: impl Into<String>, region: impl Into<Region>, origin: impl Into<Position>) -> Self {
        Self {
            res_name: res_name.into(),
            region: region.into(),
            origin: origin.into(),
        }
    }

    pub fn by_texture_ref(provider: &impl TextureRefProvider, res_name: impl Into<String>, origin: impl Into<Position>) -> GameResult<Self> {
        let res_name = res_name.into();
        let region = get_texture_region(provider, &res_name)?;
        Ok(Self::new(res_name, region, origin))
    }

    pub fn split(res_name: impl AsRef<str>, region: impl Into<Region>, cols: usize, rows: usize, origin: impl Into<Position>) -> Vec<Self> {
        let res_name = res_name.as_ref();
        let region = region.into();
        let origin = origin.into();
        let width = region.width / cols as f32;
        let height = region.height / rows as f32;
        let mut sheet = Vec::new();
        for row in 0..rows {
            for col in 0..cols {
                let region = Region::new(region.x + col as f32 * width, region.y + row as f32 * height, width, height);
                sheet.push(Self::new(res_name, region, origin));
            }
        }
        sheet
    }

    pub fn split_by_texture_ref(provider: &impl TextureRefProvider, res_name: impl AsRef<str>, cols: usize, rows: usize, origin: impl Into<Position>) -> GameResult<Vec<Self>> {
        let region = get_texture_region(provider, &res_name)?;
        Ok(Self::split(res_name, region, cols, rows, origin))
    }

    pub fn res_name(&self) -> &str {
        &self.res_name
    }

    pub fn set_res_name(&mut self, res_name: impl Into<String>) {
        self.res_name = res_name.into();
    }

    pub fn region(&self) -> Region {
        self.region
    }

    pub fn set_region(&mut self, region: impl Into<Region>) {
        self.region = region.into();
    }

    pub fn origin(&self) -> Position {
        self.origin
    }

    pub fn set_origin(&mut self, origin: impl Into<Position>) {
        self.origin = origin.into();
    }
}

pub fn get_texture_region(provider: &impl TextureRefProvider, res_name: impl AsRef<str>) -> GameResult<Region> {
    let texture_size = provider.texture_ref(res_name)?
        .texture_size()
        .ok_or_else(|| GameError::RuntimeError("no texture".into()))?;
    Ok(Region::new(0.0, 0.0, texture_size.width as f32, texture_size.height as f32))
}
