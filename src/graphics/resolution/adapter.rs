use super::ResolutionPolicy;
use tge::prelude::*;

pub trait ResolutionAdapter {
    fn policy(&self) -> ResolutionPolicy;

    fn set_policy(&mut self, policy: ResolutionPolicy);

    fn measure(&mut self, graphics: &mut Graphics);

    fn canvas_size(&self) -> Size;

    fn canvas_scale(&self) -> Vector;

    fn viewport(&self) -> Viewport;

    fn begin(&mut self, graphics: &mut Graphics);

    fn clear(&self, graphics: &mut Graphics, color: impl Into<Color>);

    fn end(&mut self, graphics: &mut Graphics);
}
