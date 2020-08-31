use tge::prelude::*;
use tge_ext::asset::*;
use tge_ext::graphics::*;
use legion::*;
use rand::Rng;

const TITLE: &str = "ECS";

mod res {
    pub const TEXTURE_CHARACTERS: &str = "assets/characters.png";
}

struct KinematicObject {
    position: Position,
    angle: Angle,
    scale: f32,
}

struct MovementComponent {
    speed: Vector,
}

fn update_movement_system(engine: &mut Engine, world: &mut World) {
    let graphics_size = engine.graphics().size();
    let delta_time_f32 = engine.timer().delta_time().as_secs_f32();
    let mut query = <(&mut KinematicObject, &mut MovementComponent)>::query();
    for (object, component) in query.iter_mut(world) {
        object.position.x += component.speed.x * delta_time_f32;
        object.position.y += component.speed.y * delta_time_f32;
        if object.position.x < 0.0 {
            object.position.x = 0.0;
            component.speed.x *= -1.0;
        }
        if object.position.x > graphics_size.width {
            object.position.x = graphics_size.width;
            component.speed.x *= -1.0;
        }
        if object.position.y < 0.0 {
            object.position.y = 0.0;
            component.speed.y *= -1.0;
        }
        if object.position.y > graphics_size.height {
            object.position.y = graphics_size.height;
            component.speed.y *= -1.0;
        }
    }
}

struct RotationComponent {
    speed: Angle,
}

fn update_rotation_system(engine: &mut Engine, world: &mut World) {
    let delta_time_f32 = engine.timer().delta_time().as_secs_f32();
    let mut query = <(&mut KinematicObject, &mut RotationComponent)>::query();
    for (object, component) in query.iter_mut(world) {
        object.angle += component.speed * delta_time_f32;
    }
}

struct ScaleComponent {
    speed: f32,
}

fn update_scale_system(engine: &mut Engine, world: &mut World) {
    let delta_time_f32 = engine.timer().delta_time().as_secs_f32();
    let mut query = <(&mut KinematicObject, &mut ScaleComponent)>::query();
    for (object, component) in query.iter_mut(world) {
        object.scale += component.speed * delta_time_f32;
        if object.scale < 1.5 {
            object.scale = 1.5;
            component.speed *= -1.0;
        }
        if object.scale > 3.0 {
            object.scale = 3.0;
            component.speed *= -1.0;
        }
    }
}

struct Role {
    animation: Animation,
}

impl Role {
    fn new(number: usize) -> Self {
        let animation = match number {
            0 => Animation::by_fps(6.0, Frame::split(res::TEXTURE_CHARACTERS, (0.0, 0.0, 128.0, 32.0), 4, 1, Position::new(16.0, 32.0))),
            1 => Animation::by_fps(6.0, Frame::split(res::TEXTURE_CHARACTERS, (0.0, 32.0, 128.0, 32.0), 4, 1, Position::new(16.0, 32.0))),
            2 => Animation::by_fps(6.0, Frame::split(res::TEXTURE_CHARACTERS, (0.0, 64.0, 128.0, 32.0), 4, 1, Position::new(16.0, 32.0))),
            _ => panic!("unknown number"),
        };
        Self {
            animation,
        }
    }
}

fn draw_role_system(graphics: &mut Graphics, registry: &AssetRegistry, world: &mut World) -> GameResult {
    let mut query = <(&Role, &KinematicObject)>::query();
    for (role, object) in query.iter(world) {
        role.animation.draw(
            graphics,
            registry,
            Transform::default()
                .scale((object.scale, object.scale))
                .rotate(object.angle)
                .translate(object.position),
        )?;
    }
    Ok(())
}

struct App {
    registry: AssetRegistry,
    world: World,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let registry = AssetRegistry::builder()
            .load::<Texture>(engine, res::TEXTURE_CHARACTERS)?
            .build();
        let mut rand = rand::thread_rng();
        let graphics_size = engine.graphics().size();
        let mut world = World::default();
        for _ in 0..40 {
            let components = {
                let number = rand.gen_range(0, 3);
                let x = rand.gen_range(0.0, graphics_size.width);
                let y = rand.gen_range(0.0, graphics_size.height);
                let angle = rand.gen_range(0.0, std::f32::consts::PI * 2.0);
                let scale = rand.gen_range(1.5, 3.0);
                let role = Role::new(number);
                let object = KinematicObject {
                    position: Position::new(x, y),
                    angle: Angle::radians(angle),
                    scale,
                };
                (role, object)
            };
            let entity = world.push(components);
            if let Some(mut entry) = world.entry(entity) {
                if rand.gen_bool(0.5) {
                    let speed_x = rand.gen_range(-100.0, 100.0);
                    let speed_y = rand.gen_range(-100.0, 100.0);
                    entry.add_component(MovementComponent {
                        speed: Vector::new(speed_x, speed_y),
                    });
                }
                if rand.gen_bool(0.5) {
                    let speed = rand.gen_range(-5.0, 5.0);
                    entry.add_component(RotationComponent {
                        speed: Angle::radians(speed),
                    });
                }
                if rand.gen_bool(0.5) {
                    let speed = rand.gen_range(-1.0, 1.0);
                    entry.add_component(ScaleComponent {
                        speed,
                    });
                }
            }
        }
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

        update_movement_system(engine, &mut self.world);
        update_rotation_system(engine, &mut self.world);
        update_scale_system(engine, &mut self.world);

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        draw_role_system(engine.graphics(), &self.registry, &mut self.world)?;

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
