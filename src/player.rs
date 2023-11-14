use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::bullet::{Bullet, BULLET_SIZE};

const PLAYER_SIZE: Vec3 = Vec3::new(20., 20., 0.);
const PLAYER_SPEED: f32 = 160.;
const INITIAL_PLAYER_POS: Vec3 = Vec3::new(0., -20., 0.);

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (movement, shoot));
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(INITIAL_PLAYER_POS).with_scale(PLAYER_SIZE),
            ..Default::default()
        },
    ));
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = transform_query.single_mut();

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.;
    }

    transform.translation.x += direction.x * PLAYER_SPEED * time.delta_seconds();
}

fn shoot(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_transform: Query<&Transform, With<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let transform = player_transform.single();

        commands.spawn((
            Bullet,
            MaterialMesh2dBundle {
                transform: transform.clone().with_scale(BULLET_SIZE),
                mesh: meshes.add(shape::Quad::default().into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                ..Default::default()
            },
        ));
    }
}
