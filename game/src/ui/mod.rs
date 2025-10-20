// bevy
use bevy::prelude::*;

// features

pub mod start_screen;
use start_screen::StartScreenPlugin;

// main UI module plugin definition

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StartScreenPlugin);
    }
}
