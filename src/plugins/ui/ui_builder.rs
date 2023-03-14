pub mod ui_builder {
    use bevy::prelude::*;

    use crate::{plugins::ui::ui_interaction_plugin::UiButton, resources::fonts::Fonts};

    #[derive(Component, Deref, DerefMut, Clone, Copy)]
    pub struct RootNode(i32);

    impl RootNode {
        pub fn new() -> Self {
            Self(rand::random())
        }
    }

    pub fn get_root_node(background_color: Color) -> NodeBundle {
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: background_color.into(),
            ..Default::default()
        }
    }

    pub fn get_container(flex_direction: FlexDirection, background_color: Color) -> NodeBundle {
        NodeBundle {
            style: Style {
                border: UiRect::all(Val::Px(2.)),
                display: Display::Flex,
                flex_direction,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(15.)),
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            background_color: background_color.into(),
            ..Default::default()
        }
    }

    pub fn add_button(
        button_component: impl Component + Copy,
        menu_button: UiButton,
        fonts: &Res<Fonts>,
    ) -> impl Fn(&mut ChildBuilder) {
        let font = fonts.regular.clone();
        move |parent| {
            parent
                .spawn(get_button(menu_button.colors.none_color))
                .insert(menu_button.clone())
                .insert(button_component)
                .with_children(|parent| {
                    parent.spawn(get_button_text(menu_button.text.clone(), font.clone()));
                });
        }
    }

    pub fn add_title(
        fonts: &Res<Fonts>,
        title: impl Into<String> + Clone,
    ) -> impl Fn(&mut ChildBuilder) {
        let font = fonts.bold.clone();
        move |parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        title.clone(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 50.,
                            color: Color::WHITE,
                        },
                    ));
                });
        }
    }

    pub fn get_button_text(text: impl Into<String>, font: Handle<Font>) -> TextBundle {
        TextBundle::from_section(
            text,
            TextStyle {
                font,
                font_size: 30.,
                color: Color::BLACK,
            },
        )
    }

    pub fn get_button(background_color: Color) -> ButtonBundle {
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.), Val::Px(80.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Px(5.)),
                ..Default::default()
            },
            background_color: background_color.into(),
            ..Default::default()
        }
    }

    pub fn get_root_node_cleanup_system(
        root_node: RootNode,
    ) -> impl Fn(Commands, Query<(Entity, &RootNode)>) {
        move |mut commands: Commands, root_query: Query<(Entity, &RootNode)>| {
            for (e, root) in root_query.iter() {
                if **root == *root_node {
                    commands.entity(e).despawn_recursive();
                }
            }
        }
    }
}
