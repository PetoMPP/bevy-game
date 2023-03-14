use std::time::Duration;

use bevy::prelude::*;

use crate::{
    plugins::sprite_animation_plugin::AnimationTimer, resources::textures::Textures, AppState,
    SPRITE_SCALE,
};

use super::movement_plugin::TIME_STEP;

const FRAMES_PER_TIME_STEP: u32 = 2;
const ANIMATION_STEP_S: f32 = TIME_STEP * FRAMES_PER_TIME_STEP as f32;

#[derive(Component)]
pub struct ExplosionInvoke {
    pub translation: Vec3,
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Playing).with_system(explosion_spawn_system),
        )
        .add_system_set(SystemSet::on_exit(AppState::Playing).with_system(cleanup_system));
    }
}

fn cleanup_system(
    mut commands: Commands,
    expl_query: Query<Entity, With<AnimationTimer>>,
    invoke_query: Query<Entity, With<ExplosionInvoke>>,
) {
    for e in expl_query.iter().chain(invoke_query.iter()) {
        commands.entity(e).despawn();
    }
}

fn explosion_spawn_system(
    mut commands: Commands,
    textures: Res<Textures>,
    query: Query<(Entity, &ExplosionInvoke)>,
) {
    for (explosion, pos) in query.iter() {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: textures.boom.texture_atlas.clone(),
                transform: Transform {
                    translation: pos.translation,
                    scale: Vec3 {
                        x: SPRITE_SCALE,
                        y: SPRITE_SCALE,
                        z: 1.,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            AnimationTimer {
                timer: Timer::new(
                    Duration::from_secs_f32(ANIMATION_STEP_S),
                    TimerMode::Repeating,
                ),
                play_once: true,
            },
        ));
        commands.entity(explosion).despawn();
    }
}
