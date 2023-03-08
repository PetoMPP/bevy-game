use bevy::prelude::*;

use crate::{
    components::root_node::RootNode,
    plugins::{
        delayed_state_switch_plugin::StateActionButton,
        ui_interaction_plugin::{UiButton, UiButtonColors},
    },
    resources::fonts::Fonts,
    AppState,
};

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_system))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup_system));
    }
}

fn setup_system(mut commands: Commands, fonts: Res<Fonts>) {
    let colors = UiButtonColors::default();
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(get_root_node())
        .insert(RootNode)
        .with_children(|parent| {
            parent
                .spawn(get_menu_container())
                .with_children(add_title(&fonts))
                .with_children(add_button(
                    StateActionButton::StartGame,
                    UiButton::new("Start game!", colors.clone()),
                    &fonts,
                ))
                .with_children(add_button(
                    StateActionButton::Exit,
                    UiButton::new("Exit to OS", colors.clone()),
                    &fonts,
                ));
        });
}

fn cleanup_system(
    mut commands: Commands,
    root_query: Query<Entity, With<RootNode>>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    for cam in camera_query.iter() {
        commands.entity(cam).despawn();
    }
    if let Ok(root) = root_query.get_single() {
        commands.entity(root).despawn_recursive();
    }
}

fn add_button(
    button: StateActionButton,
    menu_button: UiButton,
    fonts: &Res<Fonts>,
) -> impl Fn(&mut ChildBuilder) {
    let font = fonts.regular.clone();
    move |parent| {
        parent
            .spawn(get_button(menu_button.colors.none_color))
            .insert(menu_button.clone())
            .insert(button.clone())
            .with_children(|parent| {
                parent.spawn(get_button_text(menu_button.text.clone(), font.clone()));
            });
    }
}

fn add_title(fonts: &Res<Fonts>) -> impl Fn(&mut ChildBuilder) {
    let font = fonts.bold.clone();
    move |parent| {
        parent.spawn(TextBundle::from_section(
            "Welcome to the game!",
            TextStyle {
                font: font.clone(),
                font_size: 50.,
                color: Color::WHITE,
            },
        ));
    }
}

fn get_button_text(text: impl Into<String>, font: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: font.clone(),
            font_size: 30.,
            color: Color::BLACK.into(),
        },
    )
}

fn get_button(background_color: Color) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(250.), Val::Px(80.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        background_color: background_color.into(),
        ..Default::default()
    }
}

fn get_menu_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            border: UiRect::all(Val::Px(2.)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(15.)),
            margin: UiRect::all(Val::Percent(15.)),
            ..Default::default()
        },
        background_color: Color::rgba(0.75, 0.75, 0.79, 1.0).into(),
        ..Default::default()
    }
}

fn get_root_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        background_color: Color::GRAY.into(),
        ..Default::default()
    }
}
