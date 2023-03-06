use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player.png";
const PLAYER_SPRITE_SIZE: (f32, f32) = (200., 200.);
const PLAYER_FIRE_SPRITE: &str = "player_fire.png";
const PLAYER_FIRE_SPRITE_SIZE: (f32, f32) = (100., 100.);

const ENEMY_SPRITE: &str = "enemy.png";
const ENEMY_SPRITE_SIZE: (f32, f32) = (200., 200.);
const ENEMY_FIRE_SPRITE: &str = "enemy_fire.png";
const ENEMY_FIRE_SPRITE_SIZE: (f32, f32) = (100., 100.);

const BOOM_SHEET: &str = "boom.png";
const BOOM_SHEET_SIZE: (f32, f32) = (200., 200.);
const BOOM_SHEET_GRID_SIZE: (usize, usize) = (4, 4);

#[derive(Resource)]
pub struct ImageData {
    pub image: Handle<Image>,
    pub size_px: Vec2,
}

#[derive(Resource)]
pub struct TextureAtlasData {
    pub texture_atlas: Handle<TextureAtlas>,
    pub size_px: Vec2,
    pub grid_size: IVec2,
}

#[derive(Resource)]
pub struct Textures {
    pub player: ImageData,
    pub player_fire: ImageData,
    pub enemy: ImageData,
    pub enemy_fire: ImageData,
    pub boom: TextureAtlasData,
}

fn tuple_into_vec(tuple: (f32, f32)) -> Vec2 {
    Vec2 {
        x: tuple.0,
        y: tuple.1,
    }
}

impl Textures {
    pub fn init(
        asset_server: &Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            player: ImageData {
                image: asset_server.load(PLAYER_SPRITE),
                size_px: tuple_into_vec(PLAYER_SPRITE_SIZE),
            },
            player_fire: ImageData {
                image: asset_server.load(PLAYER_FIRE_SPRITE),
                size_px: tuple_into_vec(PLAYER_FIRE_SPRITE_SIZE),
            },
            enemy: ImageData {
                image: asset_server.load(ENEMY_SPRITE),
                size_px: tuple_into_vec(ENEMY_SPRITE_SIZE),
            },
            enemy_fire: ImageData {
                image: asset_server.load(ENEMY_FIRE_SPRITE),
                size_px: tuple_into_vec(ENEMY_FIRE_SPRITE_SIZE),
            },
            boom: TextureAtlasData {
                texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                    asset_server.load(BOOM_SHEET),
                    tuple_into_vec(BOOM_SHEET_SIZE),
                    BOOM_SHEET_GRID_SIZE.0,
                    BOOM_SHEET_GRID_SIZE.1,
                    None,
                    None,
                )),
                size_px: tuple_into_vec(BOOM_SHEET_SIZE),
                grid_size: IVec2 {
                    x: BOOM_SHEET_GRID_SIZE.0.try_into().unwrap_or(1),
                    y: BOOM_SHEET_GRID_SIZE.1.try_into().unwrap_or(1),
                },
            },
        }
    }
}
