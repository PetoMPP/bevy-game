use std::time::Duration;

use bevy::prelude::*;

use crate::{resources::textures::Textures, SPRITE_SCALE};

use super::sprite_animation_plugin::AnimationTimer;

#[derive(Component)]
pub struct ExplosionInvoke {
    pub translation: Vec3
}

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(explosion_spawn_system);
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
                timer: Timer::new(Duration::from_millis(50), TimerMode::Repeating),
                play_once: true,
            },
        ));
        commands.entity(explosion).despawn();
    }
}
