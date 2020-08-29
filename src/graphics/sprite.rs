use super::{Frame, get_texture_region};
use crate::asset::TextureRefProvider;
use tge::prelude::*;

#[derive(Clone)]
pub struct Sprite {
    res_name: String,
    region: Region,
    origin: Position,
    color: Color,
}

impl Sprite {
    pub fn new(res_name: impl Into<String>, region: impl Into<Region>) -> Self {
        Self {
            res_name: res_name.into(),
            region: region.into(),
            origin: Position::zero(),
            color: Color::WHITE,
        }
    }

    pub fn by_texture_ref(provider: &impl TextureRefProvider, res_name: impl Into<String>) -> GameResult<Self> {
        let res_name = res_name.into();
        let region = get_texture_region(provider, &res_name)?;
        Ok(Self::new(res_name, region))
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

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: impl Into<Color>) {
        self.color = color.into();
    }

    pub fn draw(&self, graphics: &mut Graphics, provider: &impl TextureRefProvider, transform: impl Into<Option<Transform>>) -> GameResult {
        graphics.draw_sprite(
            provider.texture_ref(&self.res_name)?,
            SpriteDrawParams::default()
                .region(self.region)
                .origin(self.origin)
                .color(self.color),
            transform,
        );
        Ok(())
    }
}

impl From<Frame> for Sprite {
    fn from(frame: Frame) -> Self {
        Self {
            res_name: frame.res_name,
            region: frame.region,
            origin: frame.origin,
            color: Color::WHITE,
        }
    }
}
