// bevy
use bevy::prelude::*;

// states
use crate::states::GameState;

// feature files
pub mod components;
pub mod systems;

// feature plugin

pub struct StartScreenPlugin;

impl Plugin for StartScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::StartScreen), systems::setup_start_screen)
            .add_systems(Update, systems::handle_start_button.run_if(in_state(GameState::StartScreen)))
            .add_systems(OnExit(GameState::StartScreen), systems::cleanup_start_screen);
    }
}
