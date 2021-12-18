use bevy::prelude::*;

pub struct Bullet {
    pub direction: Vec2,
    pub speed: f32,
    pub damage: f32,
}

pub fn spawn_bullet(
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    transform: Transform,
    bullet: Bullet,
) {
    commands
        .spawn_bundle(SpriteBundle {
            transform,
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(transform.scale.truncate()),
            ..Default::default()
        })
        .insert(Timer::from_seconds(3.0, false))
        .insert(bullet);
}

pub struct ShellsPlugin;

impl Plugin for ShellsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(handle_bullet.system())
            .add_system(handle_remove.system());
    }
}

fn handle_bullet(mut query: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in query.iter_mut() {
        transform.translation +=
            Vec3::from((bullet.direction * bullet.speed * time.delta_seconds(), 0.0));
    }
}

fn handle_remove(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Timer), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}
