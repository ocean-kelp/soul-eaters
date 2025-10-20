// bevy
use bevy::prelude::*;

// main modules imports
mod ui;

// resources
mod resources;
use resources::fonts::GameFonts;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, load_fonts.before(ui::start_screen::systems::setup_start_screen))
        .add_plugins(ui::UiPlugin)
        .run();
}

fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameFonts::new(&asset_server));
}