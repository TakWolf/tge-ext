use tge::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ResolutionAdaptParams {
    pub canvas_size: Size,
    pub canvas_scale: Vector,
    pub viewport: Viewport,
}
