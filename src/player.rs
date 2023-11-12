use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, movement);
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
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., -20., 0.)),
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

    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x -= 100. * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += 100. * time.delta_seconds();
    }
}
