use bevy::prelude::*;

use crate::{
    components::root_node::RootNode,
    plugins::{delayed_state_switch_plugin::StateActionButton, ui_interaction_plugin::UiButton},
    resources::fonts::Fonts,
    AppState,
};

#[derive(Resource, Deref, DerefMut)]
struct PauseMenuKeyBindings(pub Vec<KeyCode>);

impl Default for PauseMenuKeyBindings {
    fn default() -> Self {
        Self(vec![KeyCode::Escape, KeyCode::P])
    }
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_bindings_system)
            .add_system(playing_active_keyboard_events_system)
            .add_system_set(SystemSet::on_pause(AppState::Playing).with_system(setup_system))
            .add_system_set(SystemSet::on_resume(AppState::Playing).with_system(cleanup_system));
    }
}

fn cleanup_system(mut commands: Commands, query: Query<Entity, With<RootNode>>) {
    if let Ok(root) = query.get_single() {
        commands.entity(root).despawn_recursive();
    }
}

fn setup_system(mut commands: Commands, fonts: Res<Fonts>, bindings: Res<PauseMenuKeyBindings>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_content: AlignContent::SpaceAround,
                ..Default::default()
            },
            background_color: Color::rgba(1., 1., 1., 0.1).into(),
            ..Default::default()
        })
        .insert(RootNode)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME PAUSED!",
                TextStyle {
                    font: fonts.bold.clone(),
                    font_size: 80.,
                    color: Color::WHITE,
                },
            ));
        })
        .with_children(get_unpause_description_node(&fonts, &bindings))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        // size: Size::new(Val::Px(250.), Val::Px(80.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.)),
                        padding: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(UiButton::default_with_a(0.5))
                .insert(StateActionButton::MainMenu)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Return to main menu",
                        TextStyle {
                            font: fonts.bold.clone(),
                            font_size: 30.,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
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

fn setup_bindings_system(mut commands: Commands) {
    commands.insert_resource(PauseMenuKeyBindings::default());
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
            AppState::MainMenu => (),
            AppState::Playing => app_state.push(AppState::Paused).unwrap(),
            AppState::Paused => app_state.pop().unwrap(),
        }
        key_input.reset(*key);
    }
}
