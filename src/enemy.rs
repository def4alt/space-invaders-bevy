use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const ENEMY_SIZE: Vec3 = Vec3::new(20., 20., 0.);
const ENEMY_DOWN_SPEED: f32 = 10.;
const ENEMY_SPEED: f32 = 10.;
const ENEMY_STARTING_POS: Vec3 = Vec3::new(0., 100., 0.);

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
                ),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for i in 1..10 {
        commands.spawn((
            Enemy,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::default().into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(
                    -75. + i as f32 * 25.0,
                    ENEMY_STARTING_POS.y,
                    0.,
                ))
                .with_scale(ENEMY_SIZE),
                ..Default::default()
            },
        ));
    }
}

fn border_check(
    enemies_query: Query<&Transform, With<Enemy>>,
    mut direction: ResMut<EnemyDirection>,
) {
    for transform in &enemies_query {
        if transform.translation.x > 100.0 {
            *direction = EnemyDirection::Left;
        }

        if transform.translation.x < -100.0 {
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