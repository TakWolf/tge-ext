use tge::prelude::*;
use tge_ext::event::*;
use tge_ext::asset::*;
use tge_ext::graphics::*;

const TITLE: &str = "Transform Resolution Adapter";

mod res {
    pub const FONT_ROBOTO: &str = "assets/Roboto/Roboto-Regular.ttf";
    pub const TEXTURE_SKY: &str = "assets/sky.png";
}

struct App {
    registry: AssetRegistry,
    design_size: Size,
    resolution_adapter: TransformResolutionAdapter,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let registry = AssetRegistry::builder()
            .load::<Font>(engine, res::FONT_ROBOTO)?
            .load::<Texture>(engine, res::TEXTURE_SKY)?
            .build();
        let design_size = Size::new(320.0, 256.0);
        let resolution_adapter = TransformResolutionAdapter::new(engine.graphics(), ResolutionPolicy::Inside(design_size));
        Ok(Self {
            registry,
            design_size,
            resolution_adapter,
        })
    }

    fn draw_scene(&mut self, engine: &mut Engine) -> GameResult {
        let canvas_size = self.resolution_adapter.canvas_size();
        engine.graphics().draw_sprite(
            self.registry.texture(res::TEXTURE_SKY)?,
            SpriteDrawParams::default()
                .region((0.0, 0.0, canvas_size.width, canvas_size.height)),
            None,
        );
        engine.graphics().draw_text(
            self.registry.font(res::FONT_ROBOTO)?,
            &format!("Press Num1 ~ Num7 to switch resolution policy\nCurrent policy: {}", self.resolution_adapter.policy()),
            TextDrawParams::default()
                .color(Color::BLACK),
            None,
        );
        engine.graphics().draw_sprite(
            TextureRef::None,
            SpriteDrawParams::default()
                .region((0.0, 0.0, canvas_size.width - 10.0, canvas_size.height - 10.0))
                .color((0.0, 0.0, 1.0, 0.2)),
            Transform::default()
                .translate((5.0, 5.0)),
        );
        if let Some(position) = engine.mouse().position() {
            let position = self.resolution_adapter.convert_to_canvas_position(position);
            engine.graphics().draw_sprite(
                TextureRef::None,
                SpriteDrawParams::default()
                    .region((0.0, 0.0, 8.0, 8.0))
                    .origin((4.0, 4.0))
                    .color(Color::RED),
                Transform::default()
                    .translate(position),
            );
        }
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

    fn event(&mut self, engine: &mut Engine, event: Event) -> GameResult<bool> {
        self.handle_event(engine, event)
    }
}

impl EventHandler for App {
    fn on_keyboard_input(&mut self, _: &mut Engine, key: KeyCode, action: KeyAction, repeated: bool) -> GameResult<()> {
        if action == KeyAction::Down && !repeated {
            match key {
                KeyCode::Num1 => self.resolution_adapter.set_policy(ResolutionPolicy::Normal),
                KeyCode::Num2 => self.resolution_adapter.set_policy(ResolutionPolicy::Center(self.design_size)),
                KeyCode::Num3 => self.resolution_adapter.set_policy(ResolutionPolicy::Stretch(self.design_size)),
                KeyCode::Num4 => self.resolution_adapter.set_policy(ResolutionPolicy::Inside(self.design_size)),
                KeyCode::Num5 => self.resolution_adapter.set_policy(ResolutionPolicy::Crop(self.design_size)),
                KeyCode::Num6 => self.resolution_adapter.set_policy(ResolutionPolicy::FixedWidth(self.design_size.width)),
                KeyCode::Num7 => self.resolution_adapter.set_policy(ResolutionPolicy::FixedHeight(self.design_size.height)),
                _ => (),
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((1024.0, 600.0)))
        .build()?
        .run_with(App::new)
}
