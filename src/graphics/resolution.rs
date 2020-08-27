use tge::prelude::*;

pub struct FitResolutionParams {
    pub canvas_size: Size,
    pub canvas_scale: Vector,
    pub viewport: Viewport,
}

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
    pub fn calculate_fit_params(&self, graphics: &Graphics) -> FitResolutionParams {
        let graphics_size = graphics.size();
        match self {
            Self::Normal => {
                FitResolutionParams {
                    canvas_size: graphics_size,
                    canvas_scale: Vector::new(1.0, 1.0),
                    viewport: Viewport::position_size(Position::zero(), graphics_size),
                }
            }
            Self::Center(design_size) => {
                let design_size = *design_size;
                let viewport_position = Position::new(
                    (graphics_size.width - design_size.width) / 2.0,
                    (graphics_size.height - design_size.height) / 2.0,
                );
                FitResolutionParams {
                    canvas_size: design_size,
                    canvas_scale: Vector::new(1.0, 1.0),
                    viewport: Viewport::position_size(viewport_position, design_size),
                }
            }
            Self::Stretch(design_size) => {
                let design_size = *design_size;
                let canvas_scale = Vector::new(
                    graphics_size.width / design_size.width,
                    graphics_size.height / design_size.height,
                );
                FitResolutionParams {
                    canvas_size: design_size,
                    canvas_scale,
                    viewport: Viewport::position_size(Position::zero(), graphics_size),
                }
            }
            ResolutionPolicy::Inside(design_size) => {
                let design_size = *design_size;
                let canvas_scale;
                let viewport_size;
                let viewport_position;
                if graphics_size.width / graphics_size.height <= design_size.width / design_size.height {
                    canvas_scale = graphics_size.width / design_size.width;
                    viewport_size = Size::new(
                        graphics_size.width,
                        design_size.height * canvas_scale,
                    );
                    viewport_position = Position::new(
                        0.0,
                        (graphics_size.height - viewport_size.height) / 2.0,
                    );
                } else {
                    canvas_scale = graphics_size.height / design_size.height;
                    viewport_size = Size::new(
                        graphics_size.width * canvas_scale,
                        graphics_size.height,
                    );
                    viewport_position = Position::new(
                        (graphics_size.width - viewport_size.width) / 2.0,
                        0.0,
                    );
                }
                FitResolutionParams {
                    canvas_size: design_size,
                    canvas_scale: Vector::new(canvas_scale, canvas_scale),
                    viewport: Viewport::position_size(viewport_position, viewport_size),
                }
            }
            ResolutionPolicy::Crop(design_size) => {
                let design_size = *design_size;
                let canvas_scale;
                let viewport_size;
                let viewport_position;
                if graphics_size.width / graphics_size.height <= design_size.width / design_size.height {
                    canvas_scale = graphics_size.height / design_size.height;
                    viewport_size = Size::new(
                        graphics_size.width * canvas_scale,
                        graphics_size.height,
                    );
                    viewport_position = Position::new(
                        (graphics_size.width - viewport_size.width) / 2.0,
                        0.0,
                    );
                } else {
                    canvas_scale = graphics_size.width / design_size.width;
                    viewport_size = Size::new(
                        graphics_size.width,
                        design_size.height * canvas_scale,
                    );
                    viewport_position = Position::new(
                        0.0,
                        (graphics_size.height - viewport_size.height) / 2.0,
                    );
                }
                FitResolutionParams {
                    canvas_size: design_size,
                    canvas_scale: Vector::new(canvas_scale, canvas_scale),
                    viewport: Viewport::position_size(viewport_position, viewport_size),
                }
            }
            ResolutionPolicy::FixedWidth(design_width) => {
                let design_width = *design_width;
                let canvas_scale = graphics_size.width / design_width;
                let canvas_size = Size::new(
                    design_width,
                    graphics_size.height / canvas_scale,
                );
                FitResolutionParams {
                    canvas_size,
                    canvas_scale: Vector::new(canvas_scale, canvas_scale),
                    viewport: Viewport::position_size(Position::zero(), graphics_size),
                }
            }
            ResolutionPolicy::FixedHeight(design_height) => {
                let design_height = *design_height;
                let canvas_scale = graphics_size.height / design_height;
                let canvas_size = Size::new(
                    graphics_size.width / canvas_scale,
                    design_height,
                );
                FitResolutionParams {
                    canvas_size,
                    canvas_scale: Vector::new(canvas_scale, canvas_scale),
                    viewport: Viewport::position_size(Position::zero(), graphics_size),
                }
            }
        }
    }
}
