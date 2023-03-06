use bevy::{app::AppExit, prelude::*};

use crate::{resources::fonts::Fonts, AppState};

const BUTTON_NORMAL_RGBA: Color = Color::rgba(1., 1., 1., 0.5);
const BUTTON_HOVER_RGBA: Color = Color::rgba(1., 1., 1., 0.3);
const BUTTON_CLICK_RGBA: Color = Color::rgba(1., 1., 1., 0.8);

#[derive(Component, Clone, Copy)]
pub enum StartMenuButton {
    StartGame,
    Exit,
}

#[derive(Component)]
pub struct RootNode;

impl StartMenuButton {
    pub fn get_text(&self) -> String {
        return match &self {
            StartMenuButton::StartGame => "Start game!".to_string(),
            StartMenuButton::Exit => "Exit to windows".to_string(),
        };
    }

    pub fn on_click(
        &self,
        app_state: &mut ResMut<State<AppState>>,
        exit: &mut EventWriter<AppExit>,
    ) {
        match *self {
            StartMenuButton::StartGame => {
                app_state.set(AppState::Playing).unwrap();
            }
            StartMenuButton::Exit => {
                exit.send(AppExit);
            }
        }
    }
}

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_system))
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu).with_system(interaction_system),
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup_system));
    }
}

fn setup_system(mut commands: Commands, fonts: Res<Fonts>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(get_root_node())
        .insert(RootNode)
        .with_children(|parent| {
            parent
                .spawn(get_menu_container())
                .with_children(add_title(&fonts))
                .with_children(add_button(StartMenuButton::StartGame, &fonts))
                .with_children(add_button(StartMenuButton::Exit, &fonts));
        });
}

fn cleanup_system(mut commands: Commands, root_query: Query<Entity, With<RootNode>>, camera_query: Query<Entity, With<Camera2d>>) {
    for cam in camera_query.iter() {
        commands.entity(cam).despawn();
    }
    if let Ok(root) = root_query.get_single() {
        commands.entity(root).despawn_recursive();
    }
}

fn interaction_system(
    mut app_state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &StartMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, button) in query.iter_mut() {
        *bg_color = match *interaction {
            Interaction::Clicked => {
                button.on_click(&mut app_state, &mut exit);
                BUTTON_CLICK_RGBA.into()
            }
            Interaction::Hovered => BUTTON_HOVER_RGBA.into(),
            Interaction::None => BUTTON_NORMAL_RGBA.into(),
        }
    }
}

fn add_button(button: StartMenuButton, fonts: &Res<Fonts>) -> impl Fn(&mut ChildBuilder) {
    let font = fonts.regular.clone();
    move |parent| {
        parent
            .spawn(get_button())
            .insert(button)
            .with_children(|parent| {
                parent.spawn(get_button_text(button.get_text(), font.clone()));
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

fn get_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(250.), Val::Px(80.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        background_color: BUTTON_NORMAL_RGBA.into(),
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
        background_color: Color::rgba(1., 1., 1., 0.5).into(),
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
