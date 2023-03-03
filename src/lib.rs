use bevy::prelude::*;
use plugins::{movement_plugin::MovementPlugin, player_plugin::PlayerPlugin, enemy_plugin::EnemyPlugin, explosion_plugin::ExplosionPlugin, sprite_animation_plugin::SpriteAnimationPlugin};
use resources::{viewport_size::ViewportSize, textures::Textures};

mod components;
mod plugins;
mod resources;

pub const SPRITE_SCALE: f32 = 0.5;

pub fn run() {
    build_app().run();
}

fn build_app() -> App {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_plugins(DefaultPlugins)
        .add_plugin(SpriteAnimationPlugin {})
        .add_plugin(MovementPlugin {})
        .add_plugin(ExplosionPlugin {})
        .add_plugin(PlayerPlugin {})
        .add_plugin(EnemyPlugin {})
        .add_startup_system(setup_system);
    app
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>
) {
    // windows
    let windows = windows.get_primary_mut().unwrap();
    windows.set_resolution(1000., 600.);
    // windows.set_position(MonitorSelection::Current, IVec2 { x: 100, y: 100 });

    // camera
    commands.spawn(Camera2dBundle::default());

    // insert resources
    commands.insert_resource(ViewportSize { w: 1000., h: 600. });
    commands.insert_resource(Textures::init(asset_server, texture_atlases));
}
