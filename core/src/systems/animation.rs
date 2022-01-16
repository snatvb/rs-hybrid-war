use bevy::prelude::*;
use crate::components::*;

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationSprite)>,
) {
    for ( mut sprite, mut animation) in query.iter_mut() {
        if !animation.is_play() {
            continue;
        }

        animation.duration.tick(time.delta());

        if animation.duration.finished() {
            sprite.index = animation.next_frame();
        }
    }
}
