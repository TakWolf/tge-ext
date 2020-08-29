use super::Frame;
use crate::asset::TextureRefProvider;
use tge::prelude::*;
use std::time::Duration;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum PlayMode {
    Normal,
    Reversed,
    PingPong,
}

impl Default for PlayMode {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone)]
pub struct Animation {
    frame_duration: Duration,
    frames: Vec<Frame>,
    play_mode: PlayMode,
    current_index: usize,
    since_last_frame: Duration,
    color: Color,
}

impl Animation {
    pub fn new(frame_duration: Duration, frames: Vec<Frame>) -> Self {
        assert_frames(&frames);
        Self {
            frame_duration,
            frames,
            play_mode: PlayMode::default(),
            current_index: 0,
            since_last_frame: Duration::new(0, 0),
            color: Color::WHITE,
        }
    }

    pub fn by_fps(fps: f32, frames: Vec<Frame>) -> Self {
        Self::new(Duration::from_secs_f32(1.0 / fps), frames)
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
        self.since_last_frame = Duration::new(0, 0);
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

    pub fn set_frames(&mut self, frames: Vec<Frame>) {
        assert_frames(&frames);
        self.frames = frames;
        self.reset();
    }

    pub fn frames_len(&self) -> usize {
        self.frames.len()
    }

    pub fn one_period_frames_len(&self) -> usize {
        match self.play_mode {
            PlayMode::Normal | PlayMode::Reversed => self.frames.len(),
            PlayMode::PingPong => (self.frames.len() * 2 - 2).max(1),
        }
    }

    pub fn play_mode(&self) -> PlayMode {
        self.play_mode
    }

    pub fn set_play_mode(&mut self, play_mode: PlayMode) {
        if self.play_mode != play_mode {
            self.play_mode = play_mode;
            self.reset();
        }
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn set_current_index(&mut self, index: usize) {
        self.current_index = index % self.one_period_frames_len();
        self.since_last_frame = Duration::new(0, 0);
    }

    pub fn current_frame_index(&self) -> usize {
        match self.play_mode {
            PlayMode::Normal=> self.current_index,
            PlayMode::Reversed => self.frames.len() - self.current_index - 1,
            PlayMode::PingPong => {
                if self.current_index < self.frames.len() {
                    self.current_index
                } else {
                    self.frames.len() * 2 - self.current_index - 2
                }
            }
        }
    }

    pub fn current_frame(&self) -> &Frame {
        self.frames.get(self.current_frame_index()).expect("wrong current index")
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
            if self.current_index >= self.one_period_frames_len() {
                self.current_index = 0;
            }
        }
    }

    pub fn draw(&self, graphics: &mut Graphics, provider: &impl TextureRefProvider, transform: impl Into<Option<Transform>>) -> GameResult {
        let frame = self.current_frame();
        graphics.draw_sprite(
            provider.texture_ref(&frame.res_name)?,
            SpriteDrawParams::default()
                .region(frame.region)
                .origin(frame.origin)
                .color(self.color),
            transform,
        );
        Ok(())
    }
}

fn assert_frames(frames: &[Frame]) {
    assert!(!frames.is_empty(), "at least one frame");
}
