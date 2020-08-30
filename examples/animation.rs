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
    animation_coin_normal: Animation,
    animation_coin_reversed: Animation,
    animation_coin_ping_pong: Animation,
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
        let mut animation_coin_normal = Animation::by_fps(10.0, Frame::split(res::TEXTURE_COIN, (0.0, 0.0, 128.0, 16.0), 8, 1, Position::zero()));
        animation_coin_normal.set_play_mode(PlayMode::Normal);
        let mut animation_coin_reversed = Animation::by_fps(10.0, Frame::split(res::TEXTURE_COIN, (0.0, 0.0, 128.0, 16.0), 8, 1, Position::zero()));
        animation_coin_reversed.set_play_mode(PlayMode::Reversed);
        let mut animation_coin_ping_pong = Animation::by_fps(10.0, Frame::split(res::TEXTURE_COIN, (0.0, 0.0, 128.0, 16.0), 8, 1, Position::zero()));
        animation_coin_ping_pong.set_play_mode(PlayMode::PingPong);
        let mut animation_role_1 = Animation::by_fps(6.0, Frame::split(res::TEXTURE_CHARACTERS, (0.0, 0.0, 128.0, 32.0), 4, 1, Position::zero()));
        animation_role_1.set_repeat_count(RepeatCount::Infinite);
        let mut animation_role_2 = Animation::by_fps(6.0, Frame::split(res::TEXTURE_CHARACTERS, (0.0, 32.0, 128.0, 32.0), 4, 1, Position::zero()));
        animation_role_2.set_repeat_count(RepeatCount::Count(4));
        let mut animation_role_3 = Animation::by_fps(6.0, Frame::split(res::TEXTURE_CHARACTERS, (0.0, 64.0, 128.0, 32.0), 4, 1, Position::zero()));
        animation_role_3.set_repeat_count(RepeatCount::Count(1));
        Ok(Self {
            registry,
            animation_coin_normal,
            animation_coin_reversed,
            animation_coin_ping_pong,
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

        self.animation_coin_normal.update(engine.timer().delta_time());
        self.animation_coin_reversed.update(engine.timer().delta_time());
        self.animation_coin_ping_pong.update(engine.timer().delta_time());
        self.animation_role_1.update(engine.timer().delta_time());
        self.animation_role_2.update(engine.timer().delta_time());
        self.animation_role_3.update(engine.timer().delta_time());
        if engine.mouse().is_button_down(MouseButton::Left) {
            self.animation_role_1.reset();
            self.animation_role_2.reset();
            self.animation_role_3.reset();
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        self.animation_coin_normal.draw(
            engine.graphics(),
            &self.registry,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((128.0, 32.0)),
        )?;
        self.animation_coin_reversed.draw(
            engine.graphics(),
            &self.registry,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((192.0, 32.0)),
        )?;
        self.animation_coin_ping_pong.draw(
            engine.graphics(),
            &self.registry,
            Transform::default()
                .scale((4.0, 4.0))
                .translate((256.0, 32.0)),
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
