use tge::prelude::*;

struct App {}

impl App {
    fn new(_: &mut Engine) -> GameResult<Self> {
        // load assets
        Ok(Self {})
    }
}

impl Game for App {
    fn update(&mut self, _: &mut Engine) -> GameResult {
        // handle logic
        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLUE);
        // draw sprites
        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title("My Game")
            .inner_size((800.0, 600.0)))
        .build()?
        .run_with(App::new)
}
