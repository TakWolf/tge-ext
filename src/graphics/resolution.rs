use tge::prelude::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum ResolutionPolicy {
    Normal,
    Center(Size),
    Stretch(Size),
    Inside(Size),
    Crop(Size),
    FixedWidth(f32),
    FixedHeight(f32),
}
