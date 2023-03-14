use bevy::prelude::*;
use plugins::{
    delayed_state_switch_plugin::DelayedStateSwitchPlugin, game::game_plugin::GamePlugin,
    resources_plugin::ResourcePlugin, sprite_animation_plugin::SpriteAnimationPlugin, ui::ui_plugin::UiPlugin,
};
use resources::viewport_size::ViewportSize;

mod components;
mod plugins;
mod resources;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Playing,
    Paused,
    Settings
}

pub const SPRITE_SCALE: f32 = 0.5;

pub fn run() {
    build_app().run();
}

fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(DelayedStateSwitchPlugin)
        .add_plugin(SpriteAnimationPlugin)
        .add_plugin(ResourcePlugin)
        .add_plugin(GamePlugin)
        .add_plugin(UiPlugin)
        .add_state(AppState::MainMenu);
    app
}
