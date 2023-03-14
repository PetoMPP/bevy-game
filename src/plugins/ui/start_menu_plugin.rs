use bevy::prelude::*;

use crate::{resources::fonts::Fonts, AppState};

use super::{
    state_action_buttons_plugin::StateActionButton,
    ui_builder::ui_builder::*,
    ui_interaction_plugin::{UiButton, UiButtonColors},
};

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        let root_node = RootNode::new();
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu).with_system(get_setup_system(root_node, true)),
        )
        .add_system_set(
            SystemSet::on_resume(AppState::MainMenu)
                .with_system(get_setup_system(root_node, false)),
        )
        .add_system_set(
            SystemSet::on_pause(AppState::MainMenu)
                .with_system(get_root_node_cleanup_system(root_node)),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(get_root_node_cleanup_system(root_node))
                .with_system(cleanup_system),
        );
    }
}

fn get_setup_system(root_node: RootNode, init: bool) -> impl Fn(Commands, Res<Fonts>) {
    move |mut commands: Commands, fonts: Res<Fonts>| {
        if init {
            commands.spawn(Camera2dBundle::default());
        }
        commands
            .spawn(get_root_node(Color::GRAY))
            .insert(root_node)
            .with_children(|parent| {
                parent
                    .spawn(get_container(
                        FlexDirection::Column,
                        Color::rgba(0.75, 0.75, 0.79, 1.0),
                    ))
                    .with_children(add_title(&fonts, "Welcome to the game!"))
                    .with_children(add_button(
                        StateActionButton::StartGame,
                        UiButton::new("Start game!", UiButtonColors::default()),
                        &fonts,
                    ))
                    .with_children(add_button(
                        StateActionButton::Settings,
                        UiButton::new("Settings", UiButtonColors::default()),
                        &fonts,
                    ))
                    .with_children(add_button(
                        StateActionButton::Exit,
                        UiButton::new("Exit to OS", UiButtonColors::default()),
                        &fonts,
                    ));
            });
    }
}

fn cleanup_system(mut commands: Commands, camera_query: Query<Entity, With<Camera2d>>) {
    for cam in camera_query.iter() {
        commands.entity(cam).despawn();
    }
}
