use bevy::prelude::*;

use crate::{components::{
    sizeable::Sizeable,
    movable::{Movable, MovementViewportBehavior},
    velocity::{AngleVelocity, Velocity},
}, ViewportSize, AppState};
pub const TIME_STEP: f32 = 1. / 60.;
pub const BASE_SPEED: f32 = 250.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Playing).with_system(movement_system));
    }
}

fn movement_system(
    mut commands: Commands,
    viewport_size: Res<ViewportSize>,
    mut query: Query<(
        Entity,
        &Velocity,
        Option<&AngleVelocity>,
        &mut Transform,
        &Movable,
        &Sizeable,
    )>,
) {
    for (e, vel, ang_vel, mut trans, movable, sizeable) in query.iter_mut() {
        let velocity = match ang_vel {
            Some(ang_vel) => {
                trans.rotate_z(ang_vel.0 * TIME_STEP * 5.);
                *vel.rotate(trans.rotation)
            }
            None => **vel,
        } * TIME_STEP
            * BASE_SPEED;

        trans.translation += velocity;

        match movable.viewport_behavior {
            MovementViewportBehavior::None => (),
            MovementViewportBehavior::DespawnOnLeave => {
                if is_outside_viewport(&trans, sizeable, &viewport_size) {
                    commands.entity(e).despawn();
                }
            }
            MovementViewportBehavior::Contain => {
                if !is_contained_in_viewport(&trans, sizeable, &viewport_size) {
                    trans.translation -= velocity;
                }
            }
        };
    }
}

fn is_contained_in_viewport(
    trans: &Mut<Transform>,
    sizeable: &Sizeable,
    viewport_size: &Res<ViewportSize>,
) -> bool {
    !(trans.translation.x.abs() > (viewport_size.w - sizeable.x / 2.) / 2.
        || trans.translation.y.abs() > (viewport_size.h - sizeable.y / 2.) / 2.)
}

fn is_outside_viewport(
    trans: &Mut<Transform>,
    sizeable: &Sizeable,
    viewport_size: &Res<ViewportSize>,
) -> bool {
    trans.translation.x.abs() > (viewport_size.w + sizeable.x / 2.) / 2.
        || trans.translation.y.abs() > (viewport_size.h + sizeable.y / 2.) / 2.
}
