use super::{ResolutionAdaptParams, ResolutionPolicy, ResolutionAdapter};
use tge::prelude::*;

pub struct CanvasResolutionAdapter {
    policy: ResolutionPolicy,
    graphics_size: Size,
    params: ResolutionAdaptParams,
    canvas: Canvas,
    locked: bool,
}

impl CanvasResolutionAdapter {
    pub fn new(engine: &mut Engine, policy: ResolutionPolicy) -> GameResult<Self> {
        let graphics_size = engine.graphics().size();
        let params = policy.calculate_params(graphics_size);
        let canvas = Canvas::new(engine, (params.canvas_size.width.ceil() as u32, params.canvas_size.height.ceil() as u32))?;
        Ok(Self {
            policy,
            graphics_size,
            params,
            canvas,
            locked: false,
        })
    }
}

impl ResolutionAdapter for CanvasResolutionAdapter {
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
