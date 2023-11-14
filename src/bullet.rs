use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::enemy::Enemy;

pub const BULLET_SIZE: Vec3 = Vec3::new(2., 10., 0.);
const BULLET_SPEED: f32 = 160.;
const BULLET_TOP_BORDER: f32 = 200.;

#[derive(Component)]
pub struct Bullet;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_borders, movement, check_collision));
    }
}
fn check_borders(mut commands: Commands, bullets: Query<(Entity, &Transform), With<Bullet>>) {
    for (entity, transform) in &bullets {
        if transform.translation.y > BULLET_TOP_BORDER {
            commands.entity(entity).despawn();
        }
    }
}

fn movement(time: Res<Time>, mut bullets: Query<&mut Transform, With<Bullet>>) {
    for mut transform in &mut bullets {
        transform.translation.y += BULLET_SPEED * time.delta_seconds();
    }
}

fn check_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform) in &bullets {
        for (entity, transform) in &enemies {
            let collision = collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate(),
                transform.translation,
                transform.scale.truncate(),
            );

            if collision.is_some() {
                commands.entity(bullet_entity).despawn();
                commands.entity(entity).despawn();
            }
        }
    }
}
