use bevy::prelude::*;

use crate::resources::{fonts::Fonts, textures::Textures, viewport_size::ViewportSize};

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_required_resources);
    }
}

fn create_required_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    // windows
    let windows = windows.get_primary_mut().unwrap();
    windows.set_resolution(1000., 600.);

    // insert resources
    commands.insert_resource(ViewportSize { w: 1000., h: 600. });
    commands.insert_resource(Textures::init(&asset_server, texture_atlases));
    commands.insert_resource(Fonts::init(&asset_server));
}
