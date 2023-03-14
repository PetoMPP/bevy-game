use std::time::Duration;

use ::bevy::prelude::*;
use bevy::app::AppExit;

use crate::{
    plugins::delayed_state_switch_plugin::{StatePopCommand, StateSetCommand, StatePushCommand},
    AppState,
};

use super::ui_interaction_plugin::UiButton;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum StateActionButton {
    StartGame,
    MainMenu,
    Settings,
    Return,
    Exit,
}

#[derive(Component)]
pub struct DelayedOnclick {
    pub button: StateActionButton,
    pub timer: Timer,
}

pub struct StateActionButtonsPlugin;

impl Plugin for StateActionButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_click_system);
    }
}

fn button_click_system(
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    mut query: Query<(&Interaction, &StateActionButton, &mut UiButton), Changed<Interaction>>,
) {
    for (interaction, button, mut menu_button) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => menu_button.clicked = true,
            _ => {
                if menu_button.clicked {
                    let delay = Timer::new(Duration::from_millis(30), TimerMode::Once);
                    match *button {
                        StateActionButton::StartGame => {
                            commands.spawn_empty().insert(StateSetCommand {
                                target: AppState::Playing,
                                delay,
                            });
                        }
                        StateActionButton::MainMenu => {
                            commands.spawn_empty().insert(StatePopCommand {
                                delay: delay.clone(),
                            });
                            commands.spawn_empty().insert(StateSetCommand {
                                target: AppState::MainMenu,
                                delay,
                            });
                        }
                        StateActionButton::Exit => exit.send(AppExit),
                        StateActionButton::Settings => {
                            commands.spawn_empty().insert(StatePushCommand {
                                target: AppState::Settings,
                                delay,
                            });
                        }
                        StateActionButton::Return => {
                            commands.spawn_empty().insert(StatePopCommand {
                                delay: delay.clone(),
                            });
                        }
                    };
                }
                menu_button.clicked = false;
            }
        };
    }
}
