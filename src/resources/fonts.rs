use bevy::prelude::*;

#[derive(Resource)]
pub struct Fonts {
    pub regular: Handle<Font>,
    pub bold: Handle<Font>
}

impl Fonts {
    pub fn init(asset_server: &Res<AssetServer>) -> Self {
        Self {
            regular: asset_server.load("fonts/OpenSans/OpenSans-Regular.ttf"),
            bold: asset_server.load("fonts/OpenSans/OpenSans-Bold.ttf"),
        }
    }
}
