use bevy::prelude::*;
use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use enemy_bullet::EnemyBulletPlugin;
use player::PlayerPlugin;

mod bullet;
mod enemy;
mod enemy_bullet;
mod player;

#[derive(Resource)]
struct SpriteSheets {
    map_tiles: Handle<TextureAtlas>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(PreStartup, setup)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyBulletPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    let texture_handle = asset_server.load("atlas.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16., 8.),
        4,
        2,
        Some(Vec2::new(2., 0.)),
        Some(Vec2::new(1., 1.)),
    );
    commands.insert_resource(SpriteSheets {
        map_tiles: texture_atlases.add(texture_atlas),
    });
}
