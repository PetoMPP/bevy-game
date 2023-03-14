use bevy::prelude::*;

use crate::{
    resources::{fonts::Fonts, key_bindings::PauseMenuKeyBindings},
    AppState,
};

use super::{
    state_action_buttons_plugin::StateActionButton,
    ui_builder::ui_builder::*,
    ui_interaction_plugin::{UiButton, UiButtonColors},
};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        let root_node = RootNode::new();
        app.add_system(playing_active_keyboard_events_system)
            .add_system_set(SystemSet::on_pause(AppState::Playing).with_system(get_setup_system(root_node)))
            .add_system_set(SystemSet::on_pause(AppState::Paused).with_system(get_root_node_cleanup_system(root_node)))
            .add_system_set(SystemSet::on_resume(AppState::Paused).with_system(get_setup_system(root_node)))
            .add_system_set(SystemSet::on_resume(AppState::Playing).with_system(get_root_node_cleanup_system(root_node)));
    }
}

fn get_setup_system(root_node: RootNode) -> impl Fn(Commands, Res<Fonts>, Res<PauseMenuKeyBindings>) {
    move |mut commands: Commands, fonts: Res<Fonts>, bindings: Res<PauseMenuKeyBindings>| {
        commands
            .spawn(get_root_node(Color::rgba(1., 1., 1., 0.1)))
            .insert(root_node)
            .with_children(|parent| {
                parent
                    .spawn(get_container(
                        FlexDirection::Column,
                        Color::rgba(1., 1., 1., 0.15),
                    ))
                    .with_children(add_title(&fonts, "GAME PAUSED!"))
                    .with_children(get_unpause_description_node(&fonts, &bindings))
                    .with_children(add_button(
                        StateActionButton::Settings,
                        UiButton::new("Settings", UiButtonColors::default_with_a(0.3)),
                        &fonts,
                    ))
                    .with_children(add_button(
                        StateActionButton::MainMenu,
                        UiButton::new("Return to main menu", UiButtonColors::default_with_a(0.3)),
                        &fonts,
                    ));
            });
    }
}

fn get_unpause_description_node(
    fonts: &Res<Fonts>,
    bindings: &PauseMenuKeyBindings,
) -> impl Fn(&mut ChildBuilder) {
    let reg_style = TextStyle {
        font: fonts.italic.clone(),
        font_size: 40.,
        color: Color::WHITE,
    };
    let bold_style = TextStyle {
        font: fonts.italic_bold.clone(),
        font_size: 40.,
        color: Color::MIDNIGHT_BLUE,
    };
    let mut sections = vec![TextSection {
        value: "To unpause the game press ".to_string(),
        style: reg_style.clone(),
    }];
    for key in (*bindings).clone() {
        sections.push(TextSection {
            value: format!("{:?}", key),
            style: bold_style.clone(),
        });
        sections.push(TextSection {
            value: " or ".to_string(),
            style: reg_style.clone(),
        })
    }
    sections.pop();
    sections.push(TextSection {
        value: " key...".to_string(),
        style: reg_style,
    });

    move |parent| {
        parent.spawn(TextBundle::from_sections(sections.clone()));
    }
}

fn playing_active_keyboard_events_system(
    mut app_state: ResMut<State<AppState>>,
    bindings: Res<PauseMenuKeyBindings>,
    mut key_input: ResMut<Input<KeyCode>>,
) {
    for key in bindings.iter() {
        if !key_input.pressed(*key) {
            continue;
        }
        match app_state.current() {
            AppState::Playing => app_state.push(AppState::Paused).unwrap(),
            AppState::Paused => app_state.pop().unwrap(),
            _ => (),
        }
        key_input.reset(*key);
    }
}
