use std::time::Duration;

use bevy::{app::AppExit, prelude::*};

use crate::AppState;

use super::ui_interaction_plugin::UiButton;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum StateActionButton {
    StartGame,
    MainMenu,
    Exit,
}

#[derive(Component)]
pub struct DelayedOnclick {
    pub button: StateActionButton,
    pub timer: Timer,
}

#[derive(Component)]
pub struct StateSetCommand {
    pub target: AppState,
    pub delay: Timer,
}

#[derive(Component)]
pub struct StatePopCommand {
    pub delay: Timer,
}

pub struct DelayedStateSwitchPlugin;

impl Plugin for DelayedStateSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_click_system)
            .add_system(state_management_system);
    }
}

fn state_management_system(
    mut commands: Commands,
    time: Res<Time>,
    mut app_state: ResMut<State<AppState>>,
    mut pop_query: Query<(Entity, &mut StatePopCommand)>,
    mut set_query: Query<(Entity, &mut StateSetCommand)>,
) {
    for (entity, mut pop_command) in pop_query.iter_mut() {
        pop_command.delay.tick(time.delta());
        if pop_command.delay.just_finished() {
            app_state.pop().unwrap();
            commands.entity(entity).despawn();
            return;
        }
    }
    for (entity, mut set_command) in set_query.iter_mut() {
        set_command.delay.tick(time.delta());
        if set_command.delay.just_finished() {
            app_state.set(set_command.target).unwrap();
            commands.entity(entity).despawn();
            return;
        }
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
                    };
                }
                menu_button.clicked = false;
            }
        };
    }
}
