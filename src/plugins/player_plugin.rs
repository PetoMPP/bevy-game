use crate::{
    components::{
        movable::{Movable, MovementOptions, MovementViewportBehavior},
        projectile::{Projectile, Target},
        sizeable::Sizeable,
        velocity::{AngleVelocity, Velocity},
    },
    resources::textures::Textures,
    ViewportSize, SPRITE_SCALE,
};
use bevy::{ecs::schedule::ShouldRun, prelude::*};

use super::explosion_plugin::ExplosionInvoke;

#[derive(PartialEq)]
enum PlayerKey {
    Up,
    Down,
    Left,
    Right,
    RotateCw,
    RotateCcw,
    Fire,
}

#[derive(Resource)]
struct PlayerKeyBinding {
    up: Vec<KeyCode>,
    down: Vec<KeyCode>,
    left: Vec<KeyCode>,
    right: Vec<KeyCode>,
    rotate_cw: Vec<KeyCode>,
    rotate_ccw: Vec<KeyCode>,
    fire: Vec<KeyCode>,
}

const FIRE_COOLDOWN_S: f32 = 0.25;

#[derive(Resource, Deref, DerefMut)]
struct PlayerLastFire(pub f32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct HitPlayer;

impl PlayerKeyBinding {
    fn pressed(&self, key: Res<Input<KeyCode>>) -> Vec<PlayerKey> {
        let mut result = Vec::new();
        if self.up.iter().find(|k| key.pressed(**k)).is_some() {
            result.push(PlayerKey::Up);
        }
        if self.down.iter().find(|k| key.pressed(**k)).is_some() {
            result.push(PlayerKey::Down);
        }
        if self.left.iter().find(|k| key.pressed(**k)).is_some() {
            result.push(PlayerKey::Left);
        }
        if self.right.iter().find(|k| key.pressed(**k)).is_some() {
            result.push(PlayerKey::Right);
        }
        if self.rotate_cw.iter().find(|k| key.pressed(**k)).is_some() {
            result.push(PlayerKey::RotateCw);
        }
        if self.rotate_ccw.iter().find(|k| key.pressed(**k)).is_some() {
            result.push(PlayerKey::RotateCcw);
        }
        if self.fire.iter().find(|k| key.pressed(**k)).is_some() {
            result.push(PlayerKey::Fire);
        }
        result
    }
}

impl Default for PlayerKeyBinding {
    fn default() -> Self {
        Self {
            up: vec![KeyCode::W, KeyCode::Up],
            down: vec![KeyCode::S, KeyCode::Down],
            left: vec![KeyCode::A, KeyCode::Left],
            right: vec![KeyCode::D, KeyCode::Right],
            rotate_cw: vec![KeyCode::E],
            rotate_ccw: vec![KeyCode::Q],
            fire: vec![KeyCode::Space],
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system)
            .add_system_set(
                SystemSet::new()
                .with_system(player_fire_system)
                .with_run_criteria(player_fire_criteria)
            )
            .add_system(player_on_hit_system);
    }
}

fn player_fire_criteria(
    time: Res<Time>,
    last_fire: Res<PlayerLastFire>,
) -> ShouldRun {
    return match (time.elapsed_seconds() - **last_fire) >= FIRE_COOLDOWN_S {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}

pub fn player_spawn_system(
    mut commands: Commands,
    viewport_size: Res<ViewportSize>,
    textures: Res<Textures>,
) {
    commands.insert_resource(PlayerKeyBinding::default());
    commands.insert_resource(PlayerLastFire(0.));

    let ytrans = -viewport_size.h / 2. + textures.player.size_px.y * SPRITE_SCALE / 2.;
    commands
        .spawn(SpriteBundle {
            texture: textures.player.image.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: ytrans,
                    z: 10.,
                },
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {})
        .insert(Velocity::from(Vec2::new(0., 0.)))
        .insert(AngleVelocity(0.))
        .insert(Sizeable(textures.player.size_px))
        .insert(Movable(MovementOptions {
            viewport_behavior: MovementViewportBehavior::Contain,
        }));
}

fn player_fire_system(
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
    bindings: Res<PlayerKeyBinding>,
    textures: Res<Textures>,
    time: Res<Time>,
    mut last_fire: ResMut<PlayerLastFire>,
    query: Query<(&Transform, &Sizeable), With<Player>>,
) {
    if let Ok((player_trans, player_size)) = query.get_single() {
        let pressed = bindings.pressed(key);
        if pressed.contains(&PlayerKey::Fire) {
            let offset = *Velocity(Vec3 {
                x: player_size.x / 2. * player_trans.scale.x - 5.,
                y: player_size.x / 4. * player_trans.scale.x,
                z: 0.,
            })
            .rotate(player_trans.rotation);
            let mut spawn_fire = |offset: Vec3| {
                let trans = player_trans.translation.clone() + offset;
                commands
                    .spawn(SpriteBundle {
                        texture: textures.player_fire.image.clone(),
                        transform: Transform {
                            translation: trans,
                            scale: player_trans.scale.clone(),
                            rotation: player_trans.rotation,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Projectile(Target::Enemy))
                    .insert(Velocity(Vec3::new(0., 1.5, 0.)).rotate(player_trans.rotation))
                    .insert(Sizeable(textures.player_fire.size_px))
                    .insert(Movable(MovementOptions {
                        viewport_behavior: MovementViewportBehavior::DespawnOnLeave,
                    }));
            };
            spawn_fire(offset);

            let offset = *Velocity(Vec3 {
                x: -(player_size.x / 2. * player_trans.scale.x - 5.),
                y: player_size.x / 4. * player_trans.scale.x,
                z: 0.,
            })
            .rotate(player_trans.rotation);
            spawn_fire(offset);
            last_fire.0 = time.elapsed_seconds();
        }
    }
}

fn player_keyboard_event_system(
    key: Res<Input<KeyCode>>,
    bindings: Res<PlayerKeyBinding>,
    mut query: Query<(&mut Velocity, &mut AngleVelocity), With<Player>>,
) {
    if let Ok((mut velocity, mut angle_velocity)) = query.get_single_mut() {
        let pressed_keys = &bindings.pressed(key);

        velocity.x = match (
            pressed_keys.contains(&PlayerKey::Left),
            pressed_keys.contains(&PlayerKey::Right),
        ) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        velocity.y = match (
            pressed_keys.contains(&PlayerKey::Down),
            pressed_keys.contains(&PlayerKey::Up),
        ) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };

        angle_velocity.0 = match (
            pressed_keys.contains(&PlayerKey::RotateCw),
            pressed_keys.contains(&PlayerKey::RotateCcw),
        ) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        };
    }
}

fn player_on_hit_system(
    mut commands: Commands,
    hit_query: Query<&HitPlayer>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    if hit_query.get_single().is_ok() {
        let (player, player_trans) =  player_query.single();
        commands.spawn_empty().insert(ExplosionInvoke {
            translation: player_trans.translation,
        });
        commands.entity(player).despawn();
    }
}
