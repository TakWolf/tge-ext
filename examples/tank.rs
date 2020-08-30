use tge::prelude::*;
use tge_ext::asset::*;
use tge_ext::graphics::*;

const TITLE: &str = "Tank";

mod res {
    pub const FONT_ROBOTO: &str = "assets/Roboto/Roboto-Regular.ttf";
    pub const TEXTURE_TANK_BODY: &str = "assets/soldier-tank/body.png";
    pub const TEXTURE_TANK_TURRENT: &str = "assets/soldier-tank/turrent.png";
}

struct App {
    registry: AssetRegistry,
    resolution_adapter: TransformResolutionAdapter,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let registry = AssetRegistry::builder()
            .load::<Font>(engine, res::FONT_ROBOTO)?
            .load::<Texture>(engine, res::TEXTURE_TANK_BODY)?
            .load::<Texture>(engine, res::TEXTURE_TANK_TURRENT)?
            .build();
        let resolution_adapter = TransformResolutionAdapter::new(engine.graphics(), ResolutionPolicy::Inside(Size::new(1024.0, 600.0)));
        Ok(Self {
            registry,
            resolution_adapter,
        })
    }

    fn draw_scene(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().draw_text(
            self.registry.font(res::FONT_ROBOTO)?,
            "Tank",
            TextDrawParams::default()
                .color(Color::BLACK),
            Transform::default()
                .translate((10.0, 10.0)),
        );
        Ok(())
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);
        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);
        self.resolution_adapter.begin(engine.graphics());
        self.resolution_adapter.clear(engine.graphics(), Color::WHITE);
        self.draw_scene(engine)?;
        self.resolution_adapter.end(engine.graphics());
        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((1024.0, 600.0))
            .maximized(true))
        .build()?
        .run_with(App::new)
}
