// bevy
use bevy::prelude::*;


// A global resource holding font handles for easy access.
#[derive(Resource)]
pub struct GameFonts {
    pub josefin_sans_regular: Handle<Font>,
    pub josefin_sans_bold: Handle<Font>,
}

impl GameFonts {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            josefin_sans_regular: asset_server.load("fonts/josefin_sans/JosefinSans-Regular.ttf"),
            josefin_sans_bold: asset_server.load("fonts/josefin_sans/JosefinSans-Bold.ttf"),
        }
    }
}