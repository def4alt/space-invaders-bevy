use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{
    enemy_bullet::{EnemyBullet, ENEMY_BULLET_SIZE},
    SpriteSheets,
};

const ENEMY_SIZE: Vec3 = Vec3::new(2., 2., 0.);
const ENEMY_DOWN_SPEED: f32 = 10.;
const ENEMY_SPEED: f32 = 10.;
const ENEMY_STARTING_POS: Vec3 = Vec3::new(0., 100., 0.);
const ENEMY_BORDER: Vec3 = Vec3::new(200., 0., 0.);

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
enum EnemyDirection {
    Left,
    Right,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyDirection::Right)
            .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (
                    border_check.run_if(not(resource_changed::<EnemyDirection>())),
                    move_down.run_if(
                        resource_changed::<EnemyDirection>()
                            .and_then(|direction: Res<EnemyDirection>| !direction.is_added()),
                    ),
                    movement.run_if(not(resource_changed::<EnemyDirection>())),
                    shoot,
                ),
            );
    }
}

fn setup(mut commands: Commands, handles: Res<SpriteSheets>) {
    for i in 1..10 {
        commands.spawn((
            Enemy,
            SpriteSheetBundle {
                texture_atlas: handles.map_tiles.clone(),
                sprite: TextureAtlasSprite::new(1),
                transform: Transform::from_translation(Vec3::new(
                    -75. + i as f32 * 48.0,
                    ENEMY_STARTING_POS.y,
                    0.,
                ))
                .with_scale(ENEMY_SIZE),
                ..Default::default()
            },
        ));
    }
}

fn shoot(
    mut commands: Commands,
    enemies: Query<&Transform, With<Enemy>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut rng = rand::thread_rng();

    for transform in &enemies {
        if rng.gen_bool(1. / 30.) {
            commands.spawn((
                EnemyBullet,
                MaterialMesh2dBundle {
                    transform: transform.clone().with_scale(ENEMY_BULLET_SIZE),
                    mesh: meshes.add(shape::Quad::default().into()).into(),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    ..Default::default()
                },
            ));
        }
    }
}

fn border_check(
    enemies_query: Query<&Transform, With<Enemy>>,
    mut direction: ResMut<EnemyDirection>,
) {
    for transform in &enemies_query {
        if transform.translation.x > ENEMY_BORDER.x {
            *direction = EnemyDirection::Left;
        }

        if transform.translation.x < -ENEMY_BORDER.x {
            *direction = EnemyDirection::Right;
        }
    }
}

fn movement(
    mut enemies_query: Query<&mut Transform, With<Enemy>>,
    direction: ResMut<EnemyDirection>,
) {
    for mut transform in &mut enemies_query {
        let mut move_direction = Vec3::ZERO;

        match *direction {
            EnemyDirection::Left => move_direction.x -= 1.,
            EnemyDirection::Right => move_direction.x += 1.,
        }

        transform.translation += move_direction * ENEMY_SPEED;
    }
}

fn move_down(mut enemies_query: Query<&mut Transform, With<Enemy>>) {
    for mut transform in &mut enemies_query {
        transform.translation.y -= 1. * ENEMY_DOWN_SPEED;
    }
}
