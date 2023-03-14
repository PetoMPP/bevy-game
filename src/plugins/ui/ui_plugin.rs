use bevy::prelude::*;

use super::{
    pause_menu_plugin::PauseMenuPlugin, start_menu_plugin::StartMenuPlugin,
    state_action_buttons_plugin::StateActionButtonsPlugin,
    ui_interaction_plugin::UiInteractionPlugin, settings_menu_plugin::SettingsMenuPlugin,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiInteractionPlugin)
            .add_plugin(StateActionButtonsPlugin)
            .add_plugin(StartMenuPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_plugin(SettingsMenuPlugin);
    }
}
