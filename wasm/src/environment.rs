use bevy::prelude::*;

use crate::{materials::Materials, sprites};

pub struct Wall;

pub fn setup(
    mut commands: Commands,
    materials: Res<Materials>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    let material = materials
        .environment
        .get(sprites::PROTO.path)
        .expect("the material must be loaded");
    let wall = sprites::PROTO
        .objects
        .get("wall")
        .expect("wall must be registered in this texture");
    let mut texture_atlas = TextureAtlas::new_empty(material.clone(), sprites::PROTO.size);
    texture_atlas.add_texture(wall.as_rect());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(texture_atlas),
            transform: Transform {
                translation: Vec3::new(60.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wall);
}
