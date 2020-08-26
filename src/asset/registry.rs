use tge::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

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

    pub fn insert_once<A: Into<AssetHolder>, F: FnOnce() -> GameResult<A>>(&mut self, name: impl Into<String>, f: F) -> GameResult {
        match self.assets.entry(name.into()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(f()?.into()),
        };
        Ok(())
    }

    pub fn replace(&mut self, name: impl Into<String>, asset: impl Into<AssetHolder>) {
        self.assets.insert(name.into(), asset.into());
    }

    pub fn contains(&mut self, name: impl AsRef<str>) -> bool {
        self.assets.contains_key(name.as_ref())
    }

    pub fn remove(&mut self, name: impl AsRef<str>) -> Option<AssetHolder> {
        self.assets.remove(name.as_ref())
    }

    pub fn load_texture(&mut self, engine: &mut Engine, path: impl AsRef<str>) -> GameResult {
        let path = path.as_ref();
        self.insert(path.to_owned(), Texture::load(engine, path)?)
    }

    pub fn load_texture_once(&mut self, engine: &mut Engine, path: impl AsRef<str>) -> GameResult {
        let path = path.as_ref();
        self.insert_once(path.to_owned(), || Texture::load(engine, path))
    }

    pub fn load_font(&mut self, engine: &mut Engine, path: impl AsRef<str>) -> GameResult {
        let path = path.as_ref();
        self.insert(path.to_owned(), Font::load(engine, path)?)
    }

    pub fn load_font_once(&mut self, engine: &mut Engine, path: impl AsRef<str>) -> GameResult {
        let path = path.as_ref();
        self.insert_once(path.to_owned(), || Font::load(engine, path))
    }
}

impl ProgramProvider for AssetRegistry {
    fn program(&self, name: impl AsRef<str>) -> GameResult<&Program> {
        match self.assets.get(name.as_ref()) {
            Some(AssetHolder::Program(program)) => Ok(program),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a program".into())),
        }
    }

    fn program_mut(&mut self, name: impl AsRef<str>) -> GameResult<&mut Program> {
        match self.assets.get_mut(name.as_ref()) {
            Some(AssetHolder::Program(program)) => Ok(program),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a program".into())),
        }
    }
}

impl TextureProvider for AssetRegistry {
    fn texture(&self, name: impl AsRef<str>) -> GameResult<&Texture> {
        match self.assets.get(name.as_ref()) {
            Some(AssetHolder::Texture(texture)) => Ok(texture),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a texture".into())),
        }
    }

    fn texture_mut(&mut self, name: impl AsRef<str>) -> GameResult<&mut Texture> {
        match self.assets.get_mut(name.as_ref()) {
            Some(AssetHolder::Texture(texture)) => Ok(texture),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a texture".into())),
        }
    }
}

impl CanvasProvider for AssetRegistry {
    fn canvas(&self, name: impl AsRef<str>) -> GameResult<&Canvas> {
        match self.assets.get(name.as_ref()) {
            Some(AssetHolder::Canvas(canvas)) => Ok(canvas),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a canvas".into())),
        }
    }

    fn canvas_mut(&mut self, name: impl AsRef<str>) -> GameResult<&mut Canvas> {
        match self.assets.get_mut(name.as_ref()) {
            Some(AssetHolder::Canvas(canvas)) => Ok(canvas),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a canvas".into())),
        }
    }
}

impl FontProvider for AssetRegistry {
    fn font(&self, name: impl AsRef<str>) -> GameResult<&Font> {
        match self.assets.get(name.as_ref()) {
            Some(AssetHolder::Font(font)) => Ok(font),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a font".into())),
        }
    }

    fn font_mut(&mut self, name: impl AsRef<str>) -> GameResult<&mut Font> {
        match self.assets.get_mut(name.as_ref()) {
            Some(AssetHolder::Font(font)) => Ok(font),
            None => Err(GameError::RuntimeError("asset not exists".into())),
            _ => Err(GameError::RuntimeError("asset not a font".into())),
        }
    }
}

impl TextureRefProvider for AssetRegistry {
    fn texture_ref(&self, name: impl AsRef<str>) -> GameResult<TextureRef> {
        match self.assets.get(name.as_ref()) {
            Some(AssetHolder::Texture(texture)) => Ok(TextureRef::Texture(texture)),
            Some(AssetHolder::Canvas(canvas)) => Ok(TextureRef::Canvas(canvas)),
            Some(AssetHolder::Font(font)) => Ok(TextureRef::Font(font)),
            None => Ok(TextureRef::None),
            _ => Err(GameError::RuntimeError("asset not a texture ref".into())),
        }
    }
}
