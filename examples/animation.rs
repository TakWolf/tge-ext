use tge::prelude::*;
use tge_ext::asset::*;
use tge_ext::graphics::*;

const TITLE: &str = "Animation";

mod res {
    pub const TEXTURE_COIN: &str = "assets/coin.png";
    pub const TEXTURE_CHARACTERS: &str = "assets/characters.png";
}

struct App {
    registry: AssetRegistry,
    animation_coin: Animation,
    animation_role_1: Animation,
    animation_role_2: Animation,
    animation_role_3: Animation,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let registry = AssetRegistry::builder()
            .load::<Texture>(engine, res::TEXTURE_COIN)?
            .load::<Texture>(engine, res::TEXTURE_CHARACTERS)?
            .build();
        let animation_coin = Animation::new(
            10.0,
            Sprite::new(res::TEXTURE_COIN, (0.0, 0.0, 128.0, 16.0))
                .split(8, 1, Position::zero()),
        );
        let animation_role_1 = Animation::new(
            6.0,
            Sprite::new(res::TEXTURE_CHARACTERS, (0.0, 0.0, 128.0, 32.0))
                .split(4, 1, Position::zero()),
        );
        let animation_role_2 = Animation::new(
            8.0,
            Sprite::new(res::TEXTURE_CHARACTERS, (0.0, 32.0, 128.0, 32.0))
                .split(4, 1, Position::zero()),
        );
        let animation_role_3 = Animation::new(
            12.0,
            Sprite::new(res::TEXTURE_CHARACTERS, (0.0, 64.0, 128.0, 32.0))
                .split(4, 1, Position::zero()),
        );
        Ok(Self {
            registry,
            animation_coin,
            animation_role_1,
            animation_role_2,
            animation_role_3,
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.animation_coin.update(engine.timer().delta_time());
        self.animation_role_1.update(engine.timer().delta_time());
        self.animation_role_2.update(engine.timer().delta_time());
        self.animation_role_3.update(engine.timer().delta_time());

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        self.animation_coin.draw(
            engine.graphics(),
            &self.registry,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((128.0, 32.0)),
        )?;
        self.animation_role_1.draw(
            engine.graphics(),
            &self.registry,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((128.0, 96.0)),
        )?;
        self.animation_role_2.draw(
            engine.graphics(),
            &self.registry,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((128.0, 224.0)),
        )?;
        self.animation_role_3.draw(
            engine.graphics(),
            &self.registry,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((128.0, 352.0)),
        )?;

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
