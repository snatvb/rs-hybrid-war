use std::{collections::HashMap, time::Duration};

use benimator::SpriteSheetAnimation;
use bevy::{
    math::Vec2,
    prelude::{Handle, Texture, TextureAtlas},
};

use crate::core::math::deg_to_rad;

pub type AnimationStorage = HashMap<&'static str, SpriteSheetAnimation>;

pub struct Sprite {
    pub path: &'static str,
    offset_angle: f32,
    size: Vec2,
    pub grid: &'static [u32; 2],
    pub animations: AnimationStorage,
}

impl Sprite {
    pub fn size(&self) -> &Vec2 {
        &self.size
    }

    pub fn as_texture_atlas(&self, texture: Handle<Texture>) -> TextureAtlas {
        TextureAtlas::from_grid(texture, Vec2::new(32.0, 45.0), 8, 1)
    }

    pub fn get_offset_angle(&self) -> f32 {
        self.offset_angle
    }
}

lazy_static! {
    pub static ref PLAYER: Sprite = Sprite {
        path: "images/officer/officer_walk_strip.png",
        size: Vec2::new(32.0, 45.0),
        offset_angle: deg_to_rad(-90.0),
        grid: &[1, 8],
        animations: collection! {
          "walk" => SpriteSheetAnimation::from_range(
            0..=7,
            Duration::from_millis(100),
          ),
          "idle" => SpriteSheetAnimation::from_range(
            0..=1,
            Duration::from_millis(800),
          )
        },
    };
}
