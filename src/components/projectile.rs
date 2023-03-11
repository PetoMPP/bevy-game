use bevy::prelude::*;

#[derive(PartialEq)]
pub enum Target {
    Player,
    Enemy,
}

#[derive(Component, Deref, DerefMut)]
pub struct Projectile(pub Target);
