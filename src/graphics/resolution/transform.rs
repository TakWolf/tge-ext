use super::{ResolutionAdaptParams, ResolutionPolicy, ResolutionAdapter};
use tge::prelude::*;

pub struct TransformResolutionAdapter {
    policy: ResolutionPolicy,
    graphics_size: Size,
    params: ResolutionAdaptParams,
    transform: Transform,
    locked: bool,
}

impl TransformResolutionAdapter {
    pub fn new(graphics: &mut Graphics, policy: ResolutionPolicy) -> Self {
        let graphics_size = graphics.size();
        let params = policy.calculate_params(graphics_size);
        let transform = Transform::default()
            .scale(params.scale_factor);
        Self {
            policy,
            graphics_size,
            params,
            transform,
            locked: false,
        }
    }

    fn invalidate_params(&mut self) {
        let params = self.policy.calculate_params(self.graphics_size);
        if self.params != params {
            if self.params.scale_factor != params.scale_factor {
                self.transform = Transform::default()
                    .scale(params.scale_factor);
            }
            self.params = params;
        }
    }
}

impl ResolutionAdapter for TransformResolutionAdapter {
    fn policy(&self) -> ResolutionPolicy {
        self.policy
    }

    fn set_policy(&mut self, policy: ResolutionPolicy) {
        assert!(!self.locked, "can not change policy after `begin()` and before `end()`");
        if self.policy != policy {
            self.policy = policy;
            self.invalidate_params();
        }
    }

    fn measure(&mut self, graphics: &mut Graphics) {
        assert!(!self.locked, "can not measure after `begin()` and before `end()`");
        let graphics_size = graphics.size();
        if self.graphics_size != graphics_size {
            self.graphics_size = graphics_size;
            self.invalidate_params();
        }
    }

    fn canvas_size(&self) -> Size {
        self.params.canvas_size
    }

    fn scale_factor(&self) -> Vector {
        self.params.scale_factor
    }

    fn window_viewport(&self) -> Viewport {
        self.params.window_viewport
    }

    fn canvas_viewport(&self, graphics: &mut Graphics) -> Viewport {
        let window_viewport = graphics.viewport();
        Viewport::new(
            (window_viewport.x - self.params.window_viewport.x) / self.params.scale_factor.x,
            (window_viewport.y - self.params.window_viewport.y) / self.params.scale_factor.y,
            window_viewport.width / self.params.scale_factor.x,
            window_viewport.height / self.params.scale_factor.y,
        )
    }

    fn set_canvas_viewport(&self, graphics: &mut Graphics, canvas_viewport: Option<impl Into<Viewport>>) {
        let window_viewport = canvas_viewport.map(|canvas_viewport| {
            let canvas_viewport = canvas_viewport.into();
            Viewport::new(
                canvas_viewport.x * self.params.scale_factor.x + self.params.window_viewport.x,
                canvas_viewport.y * self.params.scale_factor.y + self.params.window_viewport.y,
                canvas_viewport.width * self.params.scale_factor.x,
                canvas_viewport.height * self.params.scale_factor.y,
            )
        }).unwrap_or(self.params.window_viewport);
        graphics.set_viewport(Some(window_viewport));
    }

    fn convert_to_canvas_position(&self, window_position: impl Into<LogicalPosition>) -> Position {
        self.params.convert_to_canvas_position(window_position)
    }

    fn convert_to_window_position(&self, canvas_position: impl Into<Position>) -> LogicalPosition {
        self.params.convert_to_window_position(canvas_position)
    }

    fn begin(&mut self, graphics: &mut Graphics) {
        assert!(!self.locked, "`end()` must be called after `begin()`");
        self.measure(graphics);
        self.locked = true;
        graphics.set_viewport(Some(self.params.window_viewport));
        graphics.set_transform(self.transform);
    }

    fn clear(&self, graphics: &mut Graphics, color: impl Into<Color>) {
        assert!(self.locked, "`clear()` can only be called after `begin()` and before `end()`");
        graphics.draw_sprite(
            TextureRef::None,
            SpriteDrawParams::default()
                .region((0.0, 0.0, self.params.canvas_size.width, self.params.canvas_size.height))
                .color(color),
            None,
        );
        graphics.flush();
    }

    fn end(&mut self, graphics: &mut Graphics) {
        assert!(self.locked, "`begin()` must be called before `end()`");
        graphics.set_transform(Transform::identity());
        graphics.set_viewport(Viewport::none());
        self.locked = false;
    }
}
