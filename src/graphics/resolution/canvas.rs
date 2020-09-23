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
    pub fn new(graphics: &mut Graphics, policy: ResolutionPolicy) -> GameResult<Self> {
        let graphics_size = graphics.size();
        let params = policy.calculate_params(graphics_size);
        let canvas = Canvas::new(graphics, (params.canvas_size.width.ceil() as u32, params.canvas_size.height.ceil() as u32))?;
        Ok(Self {
            policy,
            graphics_size,
            params,
            canvas,
            locked: false,
        })
    }

    pub fn canvas_filter(&self) -> Filter {
        self.canvas.filter()
    }

    pub fn set_canvas_filter(&mut self, filter: Filter) {
        self.canvas.set_filter(filter)
    }

    fn invalidate_params(&mut self) {
        let params = self.policy.calculate_params(self.graphics_size);
        if self.params != params {
            if self.params.canvas_size != params.canvas_size {
                self.canvas.resize((params.canvas_size.width.ceil() as u32, params.canvas_size.height.ceil() as u32));
            }
            self.params = params;
        }
    }
}

impl ResolutionAdapter for CanvasResolutionAdapter {
    fn policy(&self) -> ResolutionPolicy {
        self.policy
    }

    fn set_policy(&mut self, policy: ResolutionPolicy) {
        assert!(!self.locked, "can not change state after `begin()` and before `end()`");
        if self.policy != policy {
            self.policy = policy;
            self.invalidate_params();
        }
    }

    fn measure(&mut self, graphics: &mut Graphics) {
        assert!(!self.locked, "can not change state after `begin()` and before `end()`");
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
        graphics.viewport()
    }

    fn set_canvas_viewport(&self, graphics: &mut Graphics, canvas_viewport: Option<impl Into<Viewport>>) {
        graphics.set_viewport(canvas_viewport);
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
        graphics.set_canvas(Some(&self.canvas));
    }

    fn clear(&self, graphics: &mut Graphics, color: impl Into<Color>) {
        assert!(self.locked, "`clear()` can only be called after `begin()` and before `end()`");
        graphics.clear(color);
    }

    fn end(&mut self, graphics: &mut Graphics) {
        assert!(self.locked, "`begin()` must be called before `end()`");
        graphics.set_canvas(None);
        graphics.draw_sprite(
            &self.canvas,
            None,
            Transform::default()
                .scale(self.params.scale_factor)
                .translate(self.params.window_viewport.position()),
        );
        self.locked = false;
    }
}
