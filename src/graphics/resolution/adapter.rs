use super::ResolutionPolicy;
use tge::prelude::*;

pub trait ResolutionAdapter {
    fn policy(&self) -> ResolutionPolicy;

    fn set_policy(&mut self, policy: ResolutionPolicy);

    fn measure(&mut self, graphics: &mut Graphics);

    fn canvas_size(&self) -> Size;

    fn scale_factor(&self) -> Vector;

    fn viewport(&self) -> Viewport;

    fn set_canvas_viewport(&self, graphics: &mut Graphics, viewport: Option<impl Into<Viewport>>);

    fn convert_window_position_to_canvas_position(&self, position: impl Into<LogicalPosition>) -> Position;

    fn convert_canvas_position_to_window_position(&self, position: impl Into<Position>) -> LogicalPosition;

    fn begin(&mut self, graphics: &mut Graphics);

    fn clear(&self, graphics: &mut Graphics, color: impl Into<Color>);

    fn end(&mut self, graphics: &mut Graphics);
}
