use tge::prelude::*;
use tge_ext::event::EventHandler;

const TITLE: &str = "Event Handler";

struct App {}

impl App {
    fn new(_: &mut Engine) -> GameResult<Self> {
        Ok(Self {})
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
        Ok(())
    }

    fn event(&mut self, engine: &mut Engine, event: Event) -> GameResult<bool> {
        self.handle_event(engine, event)
    }
}

impl EventHandler for App {
    fn on_window_resize(&mut self, _: &mut Engine, size: LogicalSize) -> GameResult {
        println!("on_window_resize() - {:?}", size);
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
