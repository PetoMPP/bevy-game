use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Sizeable(pub Vec2);
