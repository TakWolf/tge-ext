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
        let transform = Transform::default().scale(params.canvas_scale);
        Self {
            policy,
            graphics_size,
            params,
            transform,
            locked: false,
        }
    }
}

impl ResolutionAdapter for TransformResolutionAdapter {
    fn policy(&self) -> ResolutionPolicy {
        self.policy
    }

    fn set_policy(&mut self, policy: ResolutionPolicy) {
        unimplemented!()
    }

    fn measure(&mut self, graphics: &mut Graphics) {
        unimplemented!()
    }

    fn canvas_size(&self) -> Size<f32> {
        unimplemented!()
    }

    fn canvas_scale(&self) -> Vector<f32> {
        unimplemented!()
    }

    fn viewport(&self) -> Viewport<f32> {
        unimplemented!()
    }

    fn clear(&self, graphics: &mut Graphics, color: impl Into<Color>) {
        unimplemented!()
    }

    fn begin(&mut self, graphics: &mut Graphics) {
        unimplemented!()
    }

    fn end(&mut self, graphics: &mut Graphics) {
        unimplemented!()
    }
}
