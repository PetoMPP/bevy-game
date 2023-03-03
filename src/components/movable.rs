use bevy::prelude::*;

pub enum MovementViewportBehavior {
    None,
    DespawnOnLeave,
    Contain,
}

impl Default for MovementViewportBehavior {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default)]
pub struct MovementOptions {
    pub viewport_behavior: MovementViewportBehavior,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Movable(pub MovementOptions);
