use benimator::*;
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

use crate::sprites;

pub struct AnimationStage {
    animation_name: &'static str,
    sprite: &'static sprites::Sprite,
}

impl AnimationStage {
    pub fn new(animation_name: &'static str, sprite: &'static sprites::Sprite) -> Self {
        Self {
            animation_name,
            sprite,
        }
    }

    pub fn change_to(&mut self, animation_name: &'static str) {
        self.animation_name = animation_name;
    }

    pub fn get_current_name(&self) -> &'static str {
        self.animation_name
    }

    pub fn get_current_animation(&self) -> Option<&'static SpriteSheetAnimation> {
        self.sprite.animations.get(self.get_current_name())
    }

    pub fn current_is(&self, animation_name: &str) -> bool {
        self.get_current_name().eq(animation_name)
    }
    pub fn current_is_not(&self, animation_name: &str) -> bool {
        !self.current_is(animation_name)
    }
}

pub fn change(
    mut commands: Commands,
    mut query: Query<(Entity, &AnimationStage), Changed<AnimationStage>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    for (entity, animation_stage) in query.iter() {
        if let Some(animation) = animation_stage.get_current_animation() {
            crate::logger::log!("Set animation: {}", animation_stage.get_current_name());
            let animation_handle = animations.add(animation.clone());
            commands.entity(entity).insert(animation_handle);
        } else {
            crate::logger::warning!("Unknown animation: {}", animation_stage.get_current_name())
        }
    }
}
