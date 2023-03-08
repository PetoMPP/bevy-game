use bevy::prelude::*;

use crate::AppState;

use super::{
    enemy_plugin::EnemyPlugin, explosion_plugin::ExplosionPlugin, movement_plugin::MovementPlugin,
    player_plugin::PlayerPlugin, pause_menu_plugin::PauseMenuPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MovementPlugin)
            .add_plugin(ExplosionPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_system_set(SystemSet::on_enter(AppState::Playing).with_system(setup_system))
            .add_system_set(SystemSet::on_exit(AppState::Playing).with_system(cleanup_system));
    }
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn cleanup_system(mut commands: Commands, query: Query<Entity, With<Camera2d>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}