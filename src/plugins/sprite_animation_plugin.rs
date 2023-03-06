use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub play_once: bool,
}

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Playing).with_system(animate_sprite));
    }
}

fn animate_sprite(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (e, mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            let len = texture_atlas.textures.len();
            sprite.index = (sprite.index + 1) % len;
            if timer.play_once && sprite.index == len - 1 {
                commands.entity(e).despawn();
            }
        }
    }
}
