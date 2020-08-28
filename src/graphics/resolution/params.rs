use tge::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ResolutionAdaptParams {
    pub canvas_size: Size,
    pub scale_factor: Vector,
    pub window_viewport: Viewport,
}

impl ResolutionAdaptParams {
    pub fn convert_to_canvas_position(&self, window_position: impl Into<LogicalPosition>) -> Position<f32> {
        let window_position = window_position.into();
        Position::new(
            (window_position.x - self.window_viewport.x) / self.scale_factor.x,
            (window_position.y - self.window_viewport.y) / self.scale_factor.y,
        )
    }

    pub fn convert_to_window_position(&self, canvas_position: impl Into<Position<f32>>) -> LogicalPosition {
        let canvas_position = canvas_position.into();
        Position::new(
            canvas_position.x * self.scale_factor.x + self.window_viewport.x,
            canvas_position.y * self.scale_factor.y + self.window_viewport.y,
        )
    }
}
