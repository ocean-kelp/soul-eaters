// bevy
use bevy::prelude::*;

// states
mod states;
use states::GameState;

// main modules imports
mod ui;
mod world;
mod player;

// resources
mod resources;
use resources::fonts::GameFonts;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, load_fonts)
        .add_plugins(ui::UiPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}

fn load_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(GameFonts::new(&asset_server));
    next_state.set(GameState::StartScreen);
}