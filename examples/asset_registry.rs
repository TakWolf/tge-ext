use tge::prelude::*;
use tge_ext::asset::*;
use tge_ext::graphics::*;

const TITLE: &str = "Asset Registry";

mod res {
    pub const TEXTURE_FERRIS: &str = "assets/ferris.png";
    pub const TEXTURE_CHARACTERS: &str = "assets/characters.png";
    pub const FONT_ROBOTO: &str = "assets/Roboto/Roboto-Regular.ttf";
}

struct App {
    registry: AssetRegistry,
    sheet: Vec<Sprite>,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let registry = AssetRegistry::builder()
            .load::<Texture>(engine, res::TEXTURE_FERRIS)?
            .load::<Texture>(engine, res::TEXTURE_CHARACTERS)?
            .load::<Font>(engine, res::FONT_ROBOTO)?
            .build();
        let sheet = Sprite::by_texture(&registry, res::TEXTURE_CHARACTERS)?
            .split(23, 4, Position::zero());
        Ok(Self {
            registry,
            sheet,
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);
        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::WHITE);

        engine.graphics().draw_sprite(
            self.registry.texture(res::TEXTURE_FERRIS)?,
            None,
            Transform::default()
                .scale((0.5, 0.5)),
        );
        engine.graphics().draw_text(
            self.registry.font(res::FONT_ROBOTO)?,
            "Hello world!",
            TextDrawParams::default()
                .text_size(20.0)
                .color(Color::BLACK),
            Transform::default()
                .translate((10.0, 10.0)),
        );
        let mut transform = Transform::default();
        for sprite in &self.sheet {
            sprite.draw(engine, &self.registry, transform)?;
            transform = transform.translate((16.0, 16.0));
        }

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((800.0, 600.0)))
        .build()?
        .run_with(App::new)
}
