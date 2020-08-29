use super::Frame;
use crate::asset::TextureRefProvider;
use tge::prelude::*;
use std::time::Duration;

#[derive(Clone)]
pub struct Animation {
    frame_duration: Duration,
    frames: Vec<Frame>,
    current_index: usize,
    since_last_frame: Duration,
    color: Color,
}

impl Animation {
    pub fn new(frame_duration: Duration, frames: Vec<Frame>) -> Self {
        Self {
            frame_duration,
            frames,
            current_index: 0,
            since_last_frame: Duration::new(0, 0),
            color: Color::WHITE,
        }
    }

    pub fn by_fps(fps: f32, frames: Vec<Frame>) -> Self {
        Self::new(Duration::from_secs_f32(1.0 / fps), frames)
    }

    pub fn frame_duration(&self) -> Duration {
        self.frame_duration
    }

    pub fn set_frame_duration(&mut self, frame_duration: Duration) {
        self.frame_duration = frame_duration;
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.frame_duration.as_secs_f32()
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.frame_duration = Duration::from_secs_f32(1.0 / fps);
    }

    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    pub fn frames_mut(&mut self) -> &mut [Frame] {
        &mut self.frames
    }

    pub fn set_frames(&mut self, frames: Vec<Frame>) {
        self.frames = frames;
    }

    pub fn frames_len(&self) -> usize {
        self.frames.len()
    }

    pub fn frame(&self, index: usize) -> Option<&Frame> {
        self.frames.get(index)
    }

    pub fn frame_mut(&mut self, index: usize) -> Option<&mut Frame> {
        self.frames.get_mut(index)
    }

    pub fn current_frame(&self) -> Option<&Frame> {
        self.frames.get(self.current_index)
    }

    pub fn current_frame_mut(&mut self) -> Option<&mut Frame> {
        self.frames.get_mut(self.current_index)
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn set_current_index(&mut self, index: usize) {
        self.current_index = index;
    }

    pub fn reset_since_last_frame(&mut self) {
        self.since_last_frame = Duration::new(0, 0);
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
        self.since_last_frame = Duration::new(0, 0);
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: impl Into<Color>) {
        self.color = color.into();
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.since_last_frame += delta_time;
        if self.since_last_frame >= self.frame_duration {
            self.since_last_frame = Duration::new(0, 0);
            self.current_index += 1;
            if self.current_index >= self.frames.len() {
                self.current_index = 0;
            }
        }
    }

    pub fn draw(&self, graphics: &mut Graphics, provider: &impl TextureRefProvider, transform: impl Into<Option<Transform>>) -> GameResult {
        if let Some(frame) = self.frames.get(self.current_index) {
            graphics.draw_sprite(
                provider.texture_ref(&frame.res_name)?,
                SpriteDrawParams::default()
                    .region(frame.region)
                    .origin(frame.origin)
                    .color(self.color),
                transform,
            );
        }
        Ok(())
    }
}
