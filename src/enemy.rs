use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Enemy;

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
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(10.0, 10.0)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(-75. + i as f32 * 15.0, 100., 0.)),
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
        match *direction {
            EnemyDirection::Left => transform.translation.x -= 10.,
            EnemyDirection::Right => transform.translation.x += 10.,
        }
    }
}

fn move_down(mut enemies_query: Query<&mut Transform, With<Enemy>>) {
    for mut transform in &mut enemies_query {
        transform.translation.y -= 10.;
    }
}
