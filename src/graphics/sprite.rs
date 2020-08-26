use crate::asset::*;
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

    pub fn by_texture(provider: &impl TextureProvider, res_name: impl Into<String>) -> GameResult<Self> {
        let res_name = res_name.into();
        let region = {
            let size = provider.texture(&res_name)?.size();
            Region::new(0.0, 0.0, size.width as f32, size.height as f32)
        };
        Ok(Self::new(res_name, region))
    }

    pub fn by_canvas(provider: &impl CanvasProvider, res_name: impl Into<String>) -> GameResult<Self> {
        let res_name = res_name.into();
        let region = {
            let size = provider.canvas(&res_name)?.size();
            Region::new(0.0, 0.0, size.width as f32, size.height as f32)
        };
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

    pub fn split(&self, cols: usize, rows: usize, origin: impl Into<Position>) -> Vec<Sprite> {
        let mut sheet = Vec::new();
        let width = self.region.width / cols as f32;
        let height = self.region.height / rows as f32;
        let origin = origin.into();
        for row in 0..rows {
            for col in 0..cols {
                sheet.push(Self {
                    res_name: self.res_name.to_owned(),
                    region: Region::new(self.region.x + col as f32 * width, self.region.y + row as f32 * height, width, height),
                    origin,
                    color: self.color,
                });
            }
        }
        sheet
    }

    pub fn draw(&self, engine: &mut Engine, provider: &impl TextureRefProvider, transform: impl Into<Option<Transform>>) -> GameResult {
        engine.graphics().draw_sprite(
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
