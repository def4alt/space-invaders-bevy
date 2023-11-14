use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::player::Player;

pub const ENEMY_BULLET_SIZE: Vec3 = Vec3::new(2., 10., 0.);
const ENEMY_BULLET_SPEED: f32 = 160.;
const ENEMY_BULLET_BOTTOM_BORDER: f32 = -50.;

#[derive(Component)]
pub struct EnemyBullet;

pub struct EnemyBulletPlugin;

impl Plugin for EnemyBulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_borders, movement, check_collision));
    }
}
fn check_borders(mut commands: Commands, bullets: Query<(Entity, &Transform), With<EnemyBullet>>) {
    for (entity, transform) in &bullets {
        if transform.translation.y < ENEMY_BULLET_BOTTOM_BORDER {
            commands.entity(entity).despawn();
        }
    }
}

fn movement(time: Res<Time>, mut bullets: Query<&mut Transform, With<EnemyBullet>>) {
    for mut transform in &mut bullets {
        transform.translation.y -= ENEMY_BULLET_SPEED * time.delta_seconds();
    }
}

fn check_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<EnemyBullet>>,
    player: Query<&Transform, With<Player>>,
) {
    for (bullet_entity, bullet_transform) in &bullets {
        let transform = player.single();

        let collision = collide(
            bullet_transform.translation,
            bullet_transform.scale.truncate(),
            transform.translation,
            transform.scale.truncate(),
        );

        if collision.is_some() {
            commands.entity(bullet_entity).despawn();
            println!("GAME OVER");
        }
    }
}
