use super::ResolutionAdaptParams;
use tge::prelude::*;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResolutionPolicy {
    Normal,
    Center(Size),
    Stretch(Size),
    Inside(Size),
    Crop(Size),
    FixedWidth(f32),
    FixedHeight(f32),
}

impl ResolutionPolicy {
    pub(crate) fn calculate_params(&self, graphics_size: Size) -> ResolutionAdaptParams {
        match self {
            Self::Normal => {
                ResolutionAdaptParams {
                    canvas_size: graphics_size,
                    canvas_scale: Vector::new(1.0, 1.0),
                    viewport: Viewport::new(0.0, 0.0, graphics_size.width, graphics_size.height),
                }
            }
            Self::Center(design_size) => {
                ResolutionAdaptParams {
                    canvas_size: *design_size,
                    canvas_scale: Vector::new(1.0, 1.0),
                    viewport: Viewport::new(
                        (graphics_size.width - design_size.width) / 2.0,
                        (graphics_size.height - design_size.height) / 2.0,
                        design_size.width,
                        design_size.height,
                    ),
                }
            }
            Self::Stretch(design_size) => {
                ResolutionAdaptParams {
                    canvas_size: *design_size,
                    canvas_scale: Vector::new(
                        graphics_size.width / design_size.width,
                        graphics_size.height / design_size.height,
                    ),
                    viewport: Viewport::new(0.0, 0.0, graphics_size.width, graphics_size.height),
                }
            }
            Self::Inside(design_size) => {
                let canvas_scale;
                let viewport_size;
                let viewport_position;
                let graphics_aspect_ratio = graphics_size.width / graphics_size.height;
                let design_aspect_ratio = design_size.width / design_size.height;
                if graphics_aspect_ratio < design_aspect_ratio {
                    canvas_scale = graphics_size.width / design_size.width;
                    viewport_size = Size::new(graphics_size.width, design_size.height * canvas_scale);
                    viewport_position = Position::new(0.0, (graphics_size.height - viewport_size.height) / 2.0);
                } else if graphics_aspect_ratio > design_aspect_ratio {
                    canvas_scale = graphics_size.height / design_size.height;
                    viewport_size = Size::new(design_size.width * canvas_scale, graphics_size.height);
                    viewport_position = Position::new((graphics_size.width - viewport_size.width) / 2.0, 0.0);
                } else {
                    canvas_scale = 1.0;
                    viewport_size = graphics_size;
                    viewport_position = Position::zero();
                }
                ResolutionAdaptParams {
                    canvas_size: *design_size,
                    canvas_scale: Vector::new(canvas_scale, canvas_scale),
                    viewport: Viewport::position_size(viewport_position, viewport_size),
                }
            }
            Self::Crop(design_size) => {
                let canvas_scale;
                let viewport_size;
                let viewport_position;
                let graphics_aspect_ratio = graphics_size.width / graphics_size.height;
                let design_aspect_ratio = design_size.width / design_size.height;
                if graphics_aspect_ratio < design_aspect_ratio {
                    canvas_scale = graphics_size.height / design_size.height;
                    viewport_size = Size::new(design_size.width * canvas_scale, graphics_size.height);
                    viewport_position = Position::new((graphics_size.width - viewport_size.width) / 2.0, 0.0);
                } else if graphics_aspect_ratio > design_aspect_ratio {
                    canvas_scale = graphics_size.width / design_size.width;
                    viewport_size = Size::new(graphics_size.width, design_size.height * canvas_scale);
                    viewport_position = Position::new(0.0, (graphics_size.height - viewport_size.height) / 2.0);
                } else {
                    canvas_scale = 1.0;
                    viewport_size = graphics_size;
                    viewport_position = Position::zero();
                }
                ResolutionAdaptParams {
                    canvas_size: *design_size,
                    canvas_scale: Vector::new(canvas_scale, canvas_scale),
                    viewport: Viewport::position_size(viewport_position, viewport_size),
                }
            }
            Self::FixedWidth(design_width) => {
                let canvas_scale = graphics_size.width / design_width;
                ResolutionAdaptParams {
                    canvas_size: Size::new(
                        *design_width,
                        graphics_size.height / canvas_scale,
                    ),
                    canvas_scale: Vector::new(canvas_scale, canvas_scale),
                    viewport: Viewport::new(0.0, 0.0, graphics_size.width, graphics_size.height),
                }
            }
            Self::FixedHeight(design_height) => {
                let canvas_scale = graphics_size.height / design_height;
                ResolutionAdaptParams {
                    canvas_size: Size::new(
                        graphics_size.width / canvas_scale,
                        *design_height,
                    ),
                    canvas_scale: Vector::new(canvas_scale, canvas_scale),
                    viewport: Viewport::position_size(Position::zero(), graphics_size),
                }
            }
        }
    }
}

impl Default for ResolutionPolicy {
    fn default() -> Self {
        Self::Normal
    }
}

impl fmt::Display for ResolutionPolicy {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(fmt, "Normal"),
            Self::Center(size) => write!(fmt, "Center({}, {})", size.width, size.height),
            Self::Stretch(size) => write!(fmt, "Stretch({}, {})", size.width, size.height),
            Self::Inside(size) => write!(fmt, "Inside({}, {})", size.width, size.height),
            Self::Crop(size) => write!(fmt, "Crop({}, {})", size.width, size.height),
            Self::FixedWidth(width) => write!(fmt, "FixedWidth({})", width),
            Self::FixedHeight(height) => write!(fmt, "FixedHeight({})", height),
        }
    }
}
