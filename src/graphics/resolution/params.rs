use tge::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ResolutionAdaptParams {
    pub canvas_size: Size,
    pub scale_factor: Vector,
    pub viewport: Viewport,
}
