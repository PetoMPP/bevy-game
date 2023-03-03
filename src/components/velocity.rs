use std::ops::Neg;

use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AngleVelocity(pub f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn rotate(self, rotation: Quat) -> Self {
        let mut angle = rotation.angle_between(Quat::from_rotation_z(0.));
        if rotation.z * rotation.w < 0. {
            angle = angle.neg();
        }
        let sin = angle.sin();
        let cos = angle.cos();

        Self(Vec3 {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: 0.,
        })
    }
}

impl From<Vec2> for Velocity {
    fn from(value: Vec2) -> Self {
        Self(Vec3::from((value, 0.)))
    }
}
