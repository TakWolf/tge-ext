use tge::prelude::*;
use tge_ext::asset::*;
use tge_ext::graphics::*;
use legion::*;

const TITLE: &str = "ECS";

mod res {
    pub const FONT_ROBOTO: &str = "assets/Roboto/Roboto-Regular.ttf";
    pub const TEXTURE_TANK_BODY: &str = "assets/soldier-tank/body.png";
    pub const TEXTURE_TANK_TURRENT: &str = "assets/soldier-tank/turrent.png";
}

struct KinematicObject {
    position: Position,
    angle: Angle,
    scale: Vector,
}

struct RotationComponent {
    angle_speed: Angle,
}

struct App {
    registry: AssetRegistry,
    world: World,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let registry = AssetRegistry::builder()
            .load::<Font>(engine, res::FONT_ROBOTO)?
            .load::<Texture>(engine, res::TEXTURE_TANK_BODY)?
            .load::<Texture>(engine, res::TEXTURE_TANK_TURRENT)?
            .build();
        let mut world = World::default();
        world.extend(vec![
            (KinematicObject {
                position: Position::new(100.0, 100.0),
                angle: Angle::zero(),
                scale: Vector::new(1.0, 1.0),
            }, RotationComponent {
                angle_speed: Angle::radians(0.02),
            }),
            (KinematicObject {
                position: Position::new(200.0, 200.0),
                angle: Angle::zero(),
                scale: Vector::new(1.0, 1.0),
            }, RotationComponent {
                angle_speed: Angle::radians(0.05),
            }),
            (KinematicObject {
                position: Position::new(300.0, 300.0),
                angle: Angle::zero(),
                scale: Vector::new(1.0, 1.0),
            }, RotationComponent {
                angle_speed: Angle::radians(0.08),
            }),
        ]);
        Ok(Self {
            registry,
            world,
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        let mut query = <(&mut KinematicObject, &RotationComponent)>::query();
        for (object, component) in query.iter_mut(&mut self.world) {
            object.angle += component.angle_speed;
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        let mut query = <&KinematicObject>::query();
        for object in query.iter(&self.world) {
            engine.graphics().draw_sprite(
                TextureRef::None,
                SpriteDrawParams::default()
                    .region((0.0, 0.0, 50.0, 50.0))
                    .origin((25.0, 25.0)),
                Transform::default()
                    .scale(object.scale)
                    .rotate(object.angle)
                    .translate(object.position),
            );
        }

        engine.graphics().draw_text(
            self.registry.font(res::FONT_ROBOTO)?,
            "Tank",
            TextDrawParams::default()
                .color(Color::WHITE),
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
            .inner_size((1024.0, 600.0)))
        .build()?
        .run_with(App::new)
}
