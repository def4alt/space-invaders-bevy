use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{enemy::Enemy, player::Player};

pub const BULLET_SIZE: Vec3 = Vec3::new(2., 10., 0.);
const BULLET_SPEED: f32 = 160.;
const BULLET_TOP_BORDER: f32 = 200.;
const BULLET_DOWN_BORDER: f32 = -200.;

#[derive(Component)]
pub enum Shooter {
    Enemy,
    Player,
}

#[derive(Component)]
pub struct CollisionBox {
    pub dimensions: Vec2,
}

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
        if transform.translation.y > BULLET_TOP_BORDER
            || transform.translation.y < BULLET_DOWN_BORDER
        {
            commands.entity(entity).despawn();
        }
    }
}

fn movement(time: Res<Time>, mut bullets: Query<(&mut Transform, &Shooter), With<Bullet>>) {
    for (mut transform, shooter) in &mut bullets {
        let direction = match shooter {
            Shooter::Enemy => 1.,
            Shooter::Player => -1.,
        };
        transform.translation.y -= direction * BULLET_SPEED * time.delta_seconds();
    }
}

fn check_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform, &Shooter), With<Bullet>>,
    entities: Query<
        (
            Entity,
            &Transform,
            &CollisionBox,
            Option<&Enemy>,
            Option<&Player>,
        ),
        Without<Bullet>,
    >,
) {
    for (bullet_entity, bullet_transform, shooter) in &bullets {
        let shooter_is_enemy = match shooter {
            Shooter::Enemy => true,
            Shooter::Player => false,
        };

        'a: for (entity, transform, collision_box, enemy, player) in &entities {
            if enemy.is_some() && shooter_is_enemy {
                continue 'a;
            } else if player.is_some() && !shooter_is_enemy {
                continue 'a;
            }

            let collision = collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate(),
                transform.translation,
                collision_box.dimensions,
            );

            if collision.is_some() {
                commands.entity(bullet_entity).despawn();
                commands.entity(entity).despawn();
            }
        }
    }
}
