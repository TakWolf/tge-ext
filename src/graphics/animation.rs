use super::Sprite;
use crate::asset::TextureRefProvider;
use tge::prelude::*;
use std::time::Duration;

#[derive(Clone)]
pub struct Animation {
    fps: f32,
    frames: Vec<Sprite>,
    current_index: usize,
    since_last_frame: Duration,
}

impl Animation {
    pub fn new(fps: f32, frames: Vec<Sprite>) -> Self {
        Self {
            fps,
            frames,
            current_index: 0,
            since_last_frame: Duration::new(0, 0),
        }
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn set_fps(&mut self, fps: f32) {
        self.fps = fps;
    }

    pub fn frames(&self) -> &[Sprite] {
        &self.frames
    }

    pub fn frames_mut(&mut self) -> &mut [Sprite] {
        &mut self.frames
    }

    pub fn set_frames(&mut self, frames: Vec<Sprite>) {
        self.frames = frames;
    }

    pub fn frames_len(&self) -> usize {
        self.frames.len()
    }

    pub fn frame(&self, index: usize) -> Option<&Sprite> {
        self.frames.get(index)
    }

    pub fn frame_mut(&mut self, index: usize) -> Option<&mut Sprite> {
        self.frames.get_mut(index)
    }

    pub fn current_frame(&self) -> Option<&Sprite> {
        self.frames.get(self.current_index)
    }

    pub fn current_frame_mut(&mut self) -> Option<&mut Sprite> {
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

    pub fn update(&mut self, delta_time: Duration) {
        self.since_last_frame += delta_time;
        if self.since_last_frame.as_secs_f32() >= 1.0 / self.fps {
            self.since_last_frame = Duration::new(0, 0);
            self.current_index += 1;
            if self.current_index >= self.frames.len() {
                self.current_index = 0;
            }
        }
    }

    pub fn draw(&self, graphics: &mut Graphics, provider: &impl TextureRefProvider, transform: impl Into<Option<Transform>>) -> GameResult {
        if let Some(frame) = self.frames.get(self.current_index) {
            frame.draw(graphics, provider, transform)?;
        }
        Ok(())
    }
}
