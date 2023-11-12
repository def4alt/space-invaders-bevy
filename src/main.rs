use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod enemy;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
