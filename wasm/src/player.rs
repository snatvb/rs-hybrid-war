use std::{
    f32::consts::{FRAC_PI_2, PI},
    ops::Range,
};

use benimator::*;
use bevy::{app::Events, input::mouse::MouseMotion, prelude::*, render::camera::Camera};
use wasm_bindgen::prelude::*;

use crate::{
    animations::AnimationStage,
    components::{Speed, Velocity, WalkAvailable},
    core::{
        camera::{CursorPosition, MainCamera},
        math::{deg_to_rad, rad_to_deg},
    },
    input::{pressed_pair, KeyDirection},
    materials::Materials,
    sprites,
};

const MAX_MOVE_RANGE: Range<f32> = -1.1f32..1.1f32;

pub struct Player;

pub fn spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    mut textures: ResMut<Assets<TextureAtlas>>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let bottom = -100.0 as f64;
    let animation_handle = animations.add(
        sprites::PLAYER
            .animations
            .get("idle")
            .expect("idle animation is require for player")
            .clone(),
    );

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(sprites::PLAYER.as_texture_atlas(materials.player.clone())),
            transform: Transform {
                translation: Vec3::new(0. as f32, (bottom + 75. / 4. + 5.) as f32, 10. as f32),
                scale: Vec3::new(1., 1., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AnimationStage::new("idle", &sprites::PLAYER))
        .insert(Player)
        .insert(Velocity::new(0., 0.))
        .insert(animation_handle)
        .insert(Play)
        .insert(WalkAvailable)
        .insert(Speed(500.));
}
pub fn movement(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut query: Query<
        (Entity, &Speed, &mut Velocity, &mut AnimationStage),
        (With<WalkAvailable>, With<Player>),
    >,
) {
    if let Ok((entity, speed, mut velocity, mut animation_stage)) = query.single_mut() {
        let direction = KeyDirection::new(
            pressed_pair(&input, (KeyCode::D, KeyCode::A)),
            pressed_pair(&input, (KeyCode::W, KeyCode::S)),
        );

        let mut entity_commands = commands.entity(entity);

        if !direction.has_direction() {
            if animation_stage.current_is("walk") {
                animation_stage.change_to("idle");
            }
            return;
        }

        if MAX_MOVE_RANGE.contains(&velocity.x) {
            velocity.x += direction.x as f32;
        }

        if MAX_MOVE_RANGE.contains(&velocity.y) {
            velocity.y += direction.y as f32;
        }

        if animation_stage.current_is_not("walk") {
            animation_stage.change_to("walk")
        }
    }
}

// pub fn rotate_by_cursor(
//     mut cursor_moved: EventReader<CursorMoved>,
//     windows: Res<Windows>,
//     q_camera: Query<(&Camera, &Transform), With<OrthographicCameraBundle>>,
//     mut query: Query<&mut Transform, With<Player>>,
// ) {
//     if let (Ok(mut transform), Ok((camera, camera_transform)), Some(cursor_position)) = (
//         query.single_mut(),
//         q_camera.single(),
//         cursor_moved.iter().last().map(|f| f.position),
//     ) {
//         // let position = camera.world_to_screen(windows, camera_transform, transform.translation);

//         // let angle = (cursor_position - position).angle_between(position);
//         // crate::logger::log!(
//         //     "({};{}) -> ({};{}) = ({})",
//         //     position.x,
//         //     position.y,
//         //     cursor_position.x,
//         //     cursor_position.y,
//         //     angle
//         // );
//         // transform.rotation = Quat::from_rotation_z(angle);
//     }
// }

pub fn rotate_by_cursor(
    query_camera: Query<&CursorPosition, With<MainCamera>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let cursor_position = query_camera.single().unwrap();
        let world_cursor_pos = Vec2::from(cursor_position.world);
        let position = transform.translation.truncate();
        let direction = world_cursor_pos - position;
        let angle = direction.angle_between(Vec2::new(1.0, 0.0));
        transform.rotation = Quat::from_rotation_z(FRAC_PI_2 - angle + deg_to_rad(-90.0));
    }
}
