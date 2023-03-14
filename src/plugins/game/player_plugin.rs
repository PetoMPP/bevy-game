use crate::{
    components::{
        movable::{Movable, MovementOptions, MovementViewportBehavior},
        sizeable::Sizeable,
        velocity::{AngleVelocity, Velocity},
    },
    plugins::delayed_state_switch_plugin::StateSetCommand,
    resources::{textures::Textures, key_bindings::{PlayerKeyBinding, PlayerKey}},
    AppState, ViewportSize, SPRITE_SCALE,
};
use bevy::prelude::*;

use super::explosion_plugin::ExplosionInvoke;

const FIRE_COOLDOWN_S: f32 = 0.25;

#[derive(Resource, Deref, DerefMut)]
struct PlayerLastFire(pub f32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct HitPlayer;

#[derive(Component)]
pub struct PlayerProjectile;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Playing).with_system(player_spawn_system))
            .add_system_set(
                SystemSet::on_update(AppState::Playing)
                    .with_system(player_fire_system)
                    .with_system(player_on_hit_system)
                    .with_system(player_keyboard_event_system),
            )
            .add_system_set(SystemSet::on_exit(AppState::Playing).with_system(cleanup_system));
    }
}

fn cleanup_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    hit_query: Query<Entity, With<HitPlayer>>,
    proj_query: Query<Entity, With<PlayerProjectile>>,
    mut last_fire: ResMut<PlayerLastFire>,
) {
    player_query
        .iter()
        .chain(hit_query.iter())
        .chain(proj_query.iter())
        .for_each(|e| {
            commands.entity(e).despawn();
        });
    last_fire.0 = 0.;
}

fn player_spawn_system(
    mut commands: Commands,
    viewport_size: Res<ViewportSize>,
    textures: Res<Textures>,
) {
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
            if (time.elapsed_seconds() - **last_fire) < FIRE_COOLDOWN_S {
                return;
            }

            let offset = *Velocity(Vec3 {
                x: player_size.x / 2. * player_trans.scale.x - 5.,
                y: player_size.x / 4. * player_trans.scale.x,
                z: 0.,
            })
            .rotate(player_trans.rotation);
            let mut spawn_fire = |offset: Vec3| {
                let trans = player_trans.translation + offset;
                commands
                    .spawn(SpriteBundle {
                        texture: textures.player_fire.image.clone(),
                        transform: Transform {
                            translation: trans,
                            scale: player_trans.scale,
                            rotation: player_trans.rotation,
                        },
                        ..Default::default()
                    })
                    .insert(PlayerProjectile)
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
    hit_query: Query<Entity, With<HitPlayer>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    // immortality on double hit yay!
    if let Ok(hit) = hit_query.get_single() {
        if let Ok((player, player_trans)) = player_query.get_single() {
            commands.spawn_empty().insert(ExplosionInvoke {
                translation: player_trans.translation,
            });
            commands.spawn_empty().insert(StateSetCommand {
                target: AppState::MainMenu,
                delay: Timer::from_seconds(2., TimerMode::Once),
            });
            commands.entity(player).despawn();
            commands.entity(hit).despawn();
        }
    }
}
