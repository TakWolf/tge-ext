use tge::prelude::*;
use tge_ext::asset::*;
use tge_ext::graphics::*;

const TITLE: &str = "Asset Registry";

mod res {
    pub const TEXTURE_SKY: &str = "assets/sky.png";
    pub const TEXTURE_CHARACTERS: &str = "assets/characters.png";
    pub const FONT_ROBOTO: &str = "assets/Roboto/Roboto-Regular.ttf";
}

struct App {
    registry: AssetRegistry,
    design_size: Size,
    resolution_adapter: CanvasResolutionAdapter,
    animation: Animation,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let registry = AssetRegistry::builder()
            .load::<Texture>(engine, res::TEXTURE_SKY)?
            .load::<Texture>(engine, res::TEXTURE_CHARACTERS)?
            .load::<Font>(engine, res::FONT_ROBOTO)?
            .build();
        let design_size = Size::new(320.0, 256.0);
        let resolution_adapter = CanvasResolutionAdapter::new(engine, ResolutionPolicy::Normal)?;
        let animation = Animation::new(
            20.0,
            Sprite::by_texture(&registry, res::TEXTURE_CHARACTERS)?
                .split(23, 4, Position::zero()),
        );
        Ok(Self {
            registry,
            design_size,
            resolution_adapter,
            animation,
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.animation.update(engine.timer().delta_time());

        if engine.keyboard().is_key_down(KeyCode::Num1) {
            self.resolution_adapter.set_policy(ResolutionPolicy::Normal);
        } else if engine.keyboard().is_key_down(KeyCode::Num2) {
            self.resolution_adapter.set_policy(ResolutionPolicy::Center(self.design_size));
        } else if engine.keyboard().is_key_down(KeyCode::Num3) {
            self.resolution_adapter.set_policy(ResolutionPolicy::Stretch(self.design_size));
        } else if engine.keyboard().is_key_down(KeyCode::Num4) {
            self.resolution_adapter.set_policy(ResolutionPolicy::Inside(self.design_size));
        } else if engine.keyboard().is_key_down(KeyCode::Num5) {
            self.resolution_adapter.set_policy(ResolutionPolicy::Crop(self.design_size));
        } else if engine.keyboard().is_key_down(KeyCode::Num6) {
            self.resolution_adapter.set_policy(ResolutionPolicy::FixedWidth(self.design_size.width));
        } else if engine.keyboard().is_key_down(KeyCode::Num7) {
            self.resolution_adapter.set_policy(ResolutionPolicy::FixedHeight(self.design_size.height));
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);
        self.resolution_adapter.begin(engine.graphics());
        self.resolution_adapter.clear(engine.graphics(), Color::BLUE);

        engine.graphics().draw_sprite(
            self.registry.texture(res::TEXTURE_SKY)?,
            None,
            None,
        );
        engine.graphics().draw_text(
            self.registry.font(res::FONT_ROBOTO)?,
            &format!("Press Num1 ~ Num7 to switch resolution policy\nCurrent policy: {}", self.resolution_adapter.policy()),
            TextDrawParams::default()
                .color(Color::BLACK),
            None,
        );
        self.animation.draw(
            engine.graphics(),
            &self.registry,
            Transform::default()
                .translate((100.0, 100.0)),
        )?;

        self.resolution_adapter.end(engine.graphics());
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
