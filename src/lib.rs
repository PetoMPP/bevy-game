use bevy::prelude::*;
use plugins::{
    enemy_plugin::EnemyPlugin, explosion_plugin::ExplosionPlugin, movement_plugin::MovementPlugin,
    player_plugin::PlayerPlugin, sprite_animation_plugin::SpriteAnimationPlugin, resources_plugin::ResourcePlugin,
};
use resources::viewport_size::ViewportSize;

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
        .add_plugin(ResourcePlugin {})
        .add_startup_system(setup_system);
    app
}

fn setup_system(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle::default());
}
