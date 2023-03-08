use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct UiButton {
    pub text: String,
    pub colors: UiButtonColors,
    pub clicked: bool,
}

impl Default for UiButton {
    fn default() -> Self {
        Self {
            text: "Click me!".to_string(),
            colors: Default::default(),
            clicked: false,
        }
    }
}

impl UiButton {
    pub fn new(text: impl Into<String>, colors: UiButtonColors) -> Self {
        Self {
            text: text.into(),
            colors,
            clicked: false,
        }
    }

    pub fn default_with_a(a: f32) -> Self {
        let mut result = Self::default();
        result.colors = UiButtonColors::default_with_a(a);
        result
    }
}

const NORMAL_COLOR: Color = Color::rgba(0.8, 0.8, 0.85, 1.);
const HOVER_COLOR: Color = Color::rgba(0.9, 0.9, 0.95, 1.);
const CLICK_COLOR: Color = Color::rgba(0.7, 0.7, 0.74, 1.);

#[derive(Clone)]
pub struct UiButtonColors {
    pub none_color: Color,
    pub hover_color: Color,
    pub click_color: Color,
}

impl Default for UiButtonColors {
    fn default() -> Self {
        Self {
            none_color: NORMAL_COLOR,
            hover_color: HOVER_COLOR,
            click_color: CLICK_COLOR,
        }
    }
}

impl UiButtonColors {
    pub fn default_with_a(a: f32) -> Self {
        let mut result = Self::default();
        result.none_color.set_a(a); 
        result.hover_color.set_a(a); 
        result.click_color.set_a(a);
        result 
    }

    pub fn get_color(&self, interaction: &Interaction) -> Color {
        return match interaction {
            Interaction::Clicked => self.click_color,
            Interaction::Hovered => self.hover_color,
            Interaction::None => self.none_color,
        };
    }
}


pub struct UiInteractionPlugin;

impl Plugin for UiInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_interaction_system);
    }
}

fn button_interaction_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &UiButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, button) in query.iter_mut() {
        *bg_color = button.colors.get_color(interaction).into();
    }
}