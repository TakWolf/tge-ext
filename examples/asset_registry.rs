use tge::prelude::*;
use tge_ext::asset::*;

const TITLE: &str = "Asset Registry";

mod res {
    pub const TEXTURE_FERRIS: &str = "assets/ferris.png";
    pub const FONT_ROBOTO: &str = "assets/Roboto/Roboto-Regular.ttf";
}

struct App {
    registry: AssetRegistry,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let registry = AssetRegistry::builder()
            .load::<Texture>(engine, res::TEXTURE_FERRIS)?
            .load::<Font>(engine, res::FONT_ROBOTO)?
            .build();
        Ok(Self {
            registry,
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
