use tge::prelude::*;

pub enum AssetHolder {
    Program(Program),
    Texture(Texture),
    Canvas(Canvas),
    Font(Font),
}

impl From<Program> for AssetHolder {
    fn from(program: Program) -> Self {
        Self::Program(program)
    }
}

impl From<Texture> for AssetHolder {
    fn from(texture: Texture) -> Self {
        Self::Texture(texture)
    }
}

impl From<Canvas> for AssetHolder {
    fn from(canvas: Canvas) -> Self {
        Self::Canvas(canvas)
    }
}

impl From<Font> for AssetHolder {
    fn from(font: Font) -> Self {
        Self::Font(font)
    }
}
