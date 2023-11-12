use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
        .insert_resource(EnemyDirection::Right)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(
            FixedUpdate,
            (
                move_down.run_if(resource_changed::<EnemyDirection>()),
                enemy_movement.run_if(not(resource_changed::<EnemyDirection>())),
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
enum EnemyDirection {
    Left,
    Right,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., -20., 0.)),
            ..Default::default()
        },
    ));

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

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = transform_query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x -= 100. * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += 100. * time.delta_seconds();
    }
}

fn enemy_movement(
    mut enemies_query: Query<&mut Transform, With<Enemy>>,
    mut direction: ResMut<EnemyDirection>,
) {
    for mut transform in &mut enemies_query {
        match *direction {
            EnemyDirection::Left => transform.translation.x -= 10.,
            EnemyDirection::Right => transform.translation.x += 10.,
        }

        if transform.translation.x > 100.0 {
            *direction = EnemyDirection::Left;
        }

        if transform.translation.x < -100.0 {
            *direction = EnemyDirection::Right;
        }
    }
}

fn move_down(mut enemies_query: Query<&mut Transform, With<Enemy>>) {
    for mut transform in &mut enemies_query {
        transform.translation.y -= 10.;
    }
}
