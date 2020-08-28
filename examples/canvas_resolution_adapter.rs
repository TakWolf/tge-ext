use tge::prelude::*;
use tge_ext::event::*;
use tge_ext::asset::*;
use tge_ext::graphics::*;

const TITLE: &str = "Canvas Resolution Adapter";

mod res {
    pub const TEXTURE_NONE: &str = "texture_none";
    pub const TEXTURE_SKY: &str = "assets/sky.png";
    pub const FONT_ROBOTO: &str = "assets/Roboto/Roboto-Regular.ttf";
}

struct App {
    registry: AssetRegistry,
    design_size: Size,
    resolution_adapter: CanvasResolutionAdapter,
    cursor: Sprite,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let registry = AssetRegistry::builder()
            .load::<Texture>(engine, res::TEXTURE_SKY)?
            .load::<Font>(engine, res::FONT_ROBOTO)?
            .build();
        let design_size = Size::new(320.0, 256.0);
        let resolution_adapter = CanvasResolutionAdapter::new(engine.graphics(), ResolutionPolicy::Normal)?;
        let mut cursor = Sprite::new(res::TEXTURE_NONE, (0.0, 0.0, 8.0, 8.0));
        cursor.set_origin((4.0, 4.0));
        cursor.set_color(Color::RED);
        Ok(Self {
            registry,
            design_size,
            resolution_adapter,
            cursor,
        })
    }

    fn draw_scene(&mut self, engine: &mut Engine) -> GameResult {
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
        if let Some(position) = engine.mouse().position() {
            let position = self.resolution_adapter.convert_to_canvas_position(position);
            self.cursor.draw(
                engine.graphics(),
                &self.registry,
                Transform::default()
                    .translate(position),
            )?;
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
        engine.graphics().clear(Color::new(0.4, 0.4, 0.4, 1.0));
        self.resolution_adapter.begin(engine.graphics());
        self.resolution_adapter.clear(engine.graphics(), Color::BLACK);
        self.draw_scene(engine)?;
        self.resolution_adapter.end(engine.graphics());
        Ok(())
    }

    fn event(&mut self, engine: &mut Engine, event: Event) -> GameResult<bool> {
        self.handle_event(engine, event)
    }
}

impl EventHandler for App {
    fn on_window_resize(&mut self, engine: &mut Engine, _: LogicalSize) -> GameResult<()> {
        self.resolution_adapter.measure(engine.graphics());
        Ok(())
    }

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
            .inner_size((800.0, 600.0)))
        .build()?
        .run_with(App::new)
}
