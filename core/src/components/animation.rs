use bevy::prelude::*;

#[derive(Debug, Clone, Component, Default)]
pub struct AnimationSprite {
    pub frames: usize,
    pub stage: AnimationSpriteStage,
    pub duration: Timer,
    pub current_frame: usize,
}

impl AnimationSprite {
    pub fn is_play(&self) -> bool {
        self.stage == AnimationSpriteStage::Play
    }

    pub fn next_frame(&mut self) -> usize {
        self.current_frame = (self.current_frame + 1) % self.frames;
        self.current_frame
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }
}


#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub enum AnimationSpriteStage {
    Idle,
    Play
}

impl Default for AnimationSpriteStage {
    fn default() -> Self {
        Self::Idle
    }
}
