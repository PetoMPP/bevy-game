use bevy::sprite::collide_aabb::collide as bevy_collide;
use bevy::{prelude::*, time::FixedTimestep};
use rand::Rng;

use crate::{
    components::{
        projectile::{Projectile, Target},
        sizeable::Sizeable,
    },
    plugins::explosion_plugin::ExplosionInvoke,
    resources::textures::Textures,
    ViewportSize, SPRITE_SCALE,
};

use super::movement_plugin::TIME_STEP;
use super::player_plugin::{HitPlayer, Player};

const INITIAL_ENEMIES_COUNT: u16 = 5;
const ENEMY_RESPAWN_DELAY: f32 = TIME_STEP * 120.;

#[derive(Component)]
pub struct EnemyRespawn;

#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, initial_enemies_spawn_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(ENEMY_RESPAWN_DELAY as f64))
                    .with_system(enemy_respawn_system),
            )
            .add_system(enemy_get_hit_system)
            .add_system(enemy_hit_player_on_collision_system);
    }
}

fn initial_enemies_spawn_system(mut commands: Commands) {
    (0..INITIAL_ENEMIES_COUNT).for_each(|_| {
        commands.spawn_empty().insert(EnemyRespawn);
    });
}

fn enemy_respawn_system(
    mut commands: Commands,
    textures: Res<Textures>,
    viewport_size: Res<ViewportSize>,
    query: Query<Entity, With<EnemyRespawn>>,
    player_query: Query<(&Sizeable, &Transform), With<Player>>,
) {
    let mut rand = rand::thread_rng();

    let mut spawn_enemy = |entity| {
        let mut get_next_trans = || {
            let w_span = viewport_size.w / 2. - textures.enemy.size_px.x * SPRITE_SCALE;
            let h_span = viewport_size.h / 2. - textures.enemy.size_px.y * SPRITE_SCALE;
            let w = rand.gen_range(-w_span..w_span);
            let h = rand.gen_range(-h_span..h_span);
            Vec3 { x: w, y: h, z: 10. }
        };
        let enemy_size = textures.enemy.size_px;
        let mut enemy_trans = get_next_trans();
        let enemy_scale = Vec3 {
            x: SPRITE_SCALE,
            y: SPRITE_SCALE,
            z: 1.,
        };

        if let Ok((player_size, player_trans)) = player_query.get_single() {
            while collide(
                player_trans.translation,
                **player_size,
                player_trans.scale,
                enemy_trans,
                enemy_size,
                enemy_scale,
            ) {
                enemy_trans = get_next_trans();
            }
        }

        commands.entity(entity).despawn();
        commands
            .spawn(SpriteBundle {
                texture: textures.enemy.image.clone(),
                transform: Transform {
                    translation: enemy_trans,
                    scale: enemy_scale,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy {})
            .insert(Sizeable(enemy_size));
    };
    if let Some(enemy) = query.iter().next() {
        spawn_enemy(enemy);
    }
}

fn enemy_hit_player_on_collision_system(
    mut commands: Commands,
    player_query: Query<(&Sizeable, &Transform), With<Player>>,
    enemy_query: Query<(&Sizeable, &Transform), With<Enemy>>,
) {
    if let Ok((player_size, player_trans)) = player_query.get_single() {
        for (enemy_size, enemy_trans) in enemy_query.iter() {
            if collide_entities(player_trans, player_size, enemy_trans, enemy_size) {
                commands.spawn_empty().insert(HitPlayer {});
            }
        }
    }
}

fn enemy_get_hit_system(
    mut commands: Commands,
    proj_query: Query<(Entity, &Sizeable, &Transform, &Projectile)>,
    enemy_query: Query<(Entity, &Sizeable, &Transform), With<Enemy>>,
) {
    let mut despawn_enemy = |enemy_entity: Entity, enemy_trans: &Transform, proj_entity: Entity| {
        commands.entity(enemy_entity).despawn();
        commands.entity(proj_entity).despawn();
        commands.spawn_empty().insert(ExplosionInvoke {
            translation: enemy_trans.translation,
        });
        commands.spawn_empty().insert(EnemyRespawn {});
    };
    for (enemy_entity, enemy_size, enemy_trans) in enemy_query.iter() {
        for (proj_entity, proj_size, proj_trans, _) in proj_query
            .iter()
            .filter(|(_, _, _, projectile)| ***projectile == Target::Enemy)
        {
            if collide_entities(proj_trans, proj_size, enemy_trans, enemy_size) {
                despawn_enemy(enemy_entity, enemy_trans, proj_entity);
                break;
            }
        }
    }
}

fn collide_entities(
    proj_trans: &Transform,
    proj_size: &Sizeable,
    enemy_trans: &Transform,
    enemy_size: &Sizeable,
) -> bool {
    collide(
        proj_trans.translation,
        **proj_size,
        proj_trans.scale,
        enemy_trans.translation,
        **enemy_size,
        enemy_trans.scale,
    )
}

fn collide(
    translation1: Vec3,
    size1: Vec2,
    scale1: Vec3,
    translation2: Vec3,
    size2: Vec2,
    scale2: Vec3,
) -> bool {
    bevy_collide(
        translation1,
        size1 * scale1.x,
        translation2,
        size2 * scale2.x,
    )
    .is_some()
}
