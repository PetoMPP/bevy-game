use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct ViewportSize {
    pub w: f32,
    pub h: f32,
}
