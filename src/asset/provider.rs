use tge::prelude::*;

pub trait ProgramProvider {
    fn program(&self, name: impl AsRef<str>) -> GameResult<&Program>;

    fn program_mut(&mut self, name: impl AsRef<str>) -> GameResult<&mut Program>;
}

pub trait TextureProvider {
    fn texture(&self, name: impl AsRef<str>) -> GameResult<&Texture>;

    fn texture_mut(&mut self, name: impl AsRef<str>) -> GameResult<&mut Texture>;
}

pub trait CanvasProvider {
    fn canvas(&self, name: impl AsRef<str>) -> GameResult<&Canvas>;

    fn canvas_mut(&mut self, name: impl AsRef<str>) -> GameResult<&mut Canvas>;
}

pub trait FontProvider {
    fn font(&self, name: impl AsRef<str>) -> GameResult<&Font>;

    fn font_mut(&mut self, name: impl AsRef<str>) -> GameResult<&mut Font>;
}

pub trait TextureRefProvider {
    fn texture_ref(&self, name: impl AsRef<str>) -> GameResult<TextureRef>;
}
