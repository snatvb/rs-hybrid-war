use std::ops::Range;

use benimator::*;
use bevy::prelude::*;

use crate::{
    animations::AnimationStage,
    components::{Speed, Velocity, WalkAvailable},
    core::{
        camera::{CursorPosition, MainCamera},
        collision::{Circle, Collider, Rectangle, RigidBody},
        extensions::LookAt,
    },
    input::{pressed_pair, KeyDirection},
    materials::Materials,
    shells::{spawn_bullet, Bullet},
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
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody {
            // collider: Collider::Rectangle(Rectangle::from_size_vec(sprites::PLAYER.size)),
            collider: Collider::Circle(Circle(Vec2::new(0.0, 0.0), 20.0)),
            solid: true,
        })
        .insert(AnimationStage::new("idle", &sprites::PLAYER))
        .insert(Player)
        .insert(Velocity::default())
        .insert(animation_handle)
        .insert(Play)
        .insert(WalkAvailable)
        .insert(LookAt::default())
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

pub fn rotate_by_cursor(
    query_camera: Query<&CursorPosition, With<MainCamera>>,
    mut query: Query<&mut LookAt, With<Player>>,
) {
    if let Ok(mut look_at) = query.single_mut() {
        let cursor_position = query_camera.single().expect("Camera must be initialized");
        let world_cursor_pos = Vec2::from(cursor_position.world);
        look_at.target = world_cursor_pos;
        look_at.angle_offset = sprites::PLAYER.get_offset_angle();
    }
}

pub fn fire(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    query_camera: Query<&CursorPosition, With<MainCamera>>,
    mut query: Query<&Transform, With<Player>>,
) {
    if let Ok(transform) = query.single() {
        if mouse_input.pressed(MouseButton::Left) {
            let cursor_position = query_camera.single().expect("Camera must be initialized");
            let bullet_transform = Transform {
                translation: transform.translation.clone(),
                scale: Vec3::new(1., 1., 1.),
                ..Default::default()
            };

            let bullet = Bullet {
                direction: (Vec2::from(cursor_position.world) - transform.translation.truncate())
                    .normalize(),
                speed: 500.0,
                damage: 1.0,
            };
            spawn_bullet(&mut commands, &mut materials, bullet_transform, bullet);
        }
    }
}
