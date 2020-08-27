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
            .scale(params.canvas_scale);
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
            if self.params.canvas_scale != params.canvas_scale {
                self.transform = Transform::default()
                    .scale(params.canvas_scale);
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

    fn canvas_scale(&self) -> Vector {
        self.params.canvas_scale
    }

    fn viewport(&self) -> Viewport {
        self.params.viewport
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
    }

    fn begin(&mut self, graphics: &mut Graphics) {
        assert!(!self.locked, "`end()` must be called after `begin()`");
        self.measure(graphics);
        self.locked = true;
        graphics.set_viewport(Some(self.params.viewport));
        graphics.set_transform(self.transform);
    }

    fn end(&mut self, graphics: &mut Graphics) {
        assert!(self.locked, "`begin()` must be called before `end()`");
        graphics.set_transform(Transform::identity());
        graphics.set_viewport(Viewport::none());
        self.locked = false;
    }
}
