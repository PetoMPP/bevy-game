use bevy::prelude::*;

use crate::{resources::fonts::Fonts, AppState};

use super::{
    state_action_buttons_plugin::StateActionButton,
    ui_builder::ui_builder::*,
    ui_interaction_plugin::{UiButton, UiButtonColors},
};

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        let root_node = RootNode::new();
        app.add_system_set(
            SystemSet::on_enter(AppState::Settings).with_system(get_setup_system(root_node)),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Settings)
                .with_system(get_root_node_cleanup_system(root_node)),
        );
    }
}

fn get_setup_system(root_node: RootNode) -> impl Fn(Commands, Res<Fonts>) {
    move |mut commands: Commands, fonts: Res<Fonts>| {
        commands
            .spawn(get_root_node(Color::GRAY))
            .insert(root_node)
            .with_children(|parent| {
                parent
                    .spawn(get_container(
                        FlexDirection::Column,
                        Color::rgba(0.75, 0.75, 0.79, 1.0),
                    ))
                    .with_children(add_title(&fonts, "Sample text"))
                    .with_children(add_button(
                        StateActionButton::Return,
                        UiButton::new("Go back..", UiButtonColors::default()),
                        &fonts,
                    ));
            });
    }
}
