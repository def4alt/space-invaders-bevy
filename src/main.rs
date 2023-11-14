use bevy::prelude::*;
use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use enemy_bullet::EnemyBulletPlugin;
use player::PlayerPlugin;

mod bullet;
mod enemy;
mod enemy_bullet;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyBulletPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
