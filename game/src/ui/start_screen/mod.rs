// bevy
use bevy::prelude::*;

// feature files
pub mod systems;

// feature plugin

pub struct StartScreenPlugin;

impl Plugin for StartScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_start_screen);
    }
}
