use tge::prelude::*;
use std::collections::HashMap;

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

pub trait ProgramProvider {
    fn program(&mut self, name: impl AsRef<str>) -> GameResult<&mut Program>;
}

pub trait TextureProvider {
    fn texture(&mut self, name: impl AsRef<str>) -> GameResult<&mut Texture>;
}

pub trait CanvasProvider {
    fn canvas(&mut self, name: impl AsRef<str>) -> GameResult<&mut Canvas>;
}

pub trait FontProvider {
    fn font(&mut self, name: impl AsRef<str>) -> GameResult<&mut Font>;
}

pub trait TextureRefProvider {
    fn texture_ref(&mut self, name: impl AsRef<str>) -> GameResult<TextureRef>;
}

pub struct AssetRegistry {
    assets: HashMap<String, AssetHolder>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: impl Into<String>, asset: impl Into<AssetHolder>) -> GameResult {
        if self.assets.insert(name.into(), asset.into()).is_some() {
            return Err(GameError::RuntimeError("the asset already exists".into()));
        }
        Ok(())
    }

    pub fn replace(&mut self, name: impl Into<String>, asset: impl Into<AssetHolder>) {
        self.assets.insert(name.into(), asset.into());
    }

    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<AssetHolder> {
        self.assets.remove(name.as_ref())
    }

    pub fn with(mut self, name: impl Into<String>, asset: impl Into<AssetHolder>) -> GameResult<Self> {
        self.insert(name, asset)?;
        Ok(self)
    }
}

impl ProgramProvider for AssetRegistry {
    fn program(&mut self, name: impl AsRef<str>) -> GameResult<&mut Program> {
        match self.assets.get_mut(name.as_ref()) {
            Some(AssetHolder::Program(program)) => Ok(program),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a program".into())),
        }
    }
}

impl TextureProvider for AssetRegistry {
    fn texture(&mut self, name: impl AsRef<str>) -> GameResult<&mut Texture> {
        match self.assets.get_mut(name.as_ref()) {
            Some(AssetHolder::Texture(texture)) => Ok(texture),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a texture".into())),
        }
    }
}

impl CanvasProvider for AssetRegistry {
    fn canvas(&mut self, name: impl AsRef<str>) -> GameResult<&mut Canvas> {
        match self.assets.get_mut(name.as_ref()) {
            Some(AssetHolder::Canvas(canvas)) => Ok(canvas),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a canvas".into())),
        }
    }
}

impl FontProvider for AssetRegistry {
    fn font(&mut self, name: impl AsRef<str>) -> GameResult<&mut Font> {
        match self.assets.get_mut(name.as_ref()) {
            Some(AssetHolder::Font(font)) => Ok(font),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a font".into())),
        }
    }
}

impl TextureRefProvider for AssetRegistry {
    fn texture_ref(&mut self, name: impl AsRef<str>) -> GameResult<TextureRef> {
        match self.assets.get_mut(name.as_ref()) {
            Some(AssetHolder::Texture(texture)) => Ok(TextureRef::Texture(texture)),
            Some(AssetHolder::Canvas(canvas)) => Ok(TextureRef::Canvas(canvas)),
            Some(AssetHolder::Font(font)) => Ok(TextureRef::Font(font)),
            None => Ok(TextureRef::None),
            _ => Err(GameError::RuntimeError("asset not a texture ref".into())),
        }
    }
}
