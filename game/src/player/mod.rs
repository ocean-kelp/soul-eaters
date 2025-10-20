// bevy
use bevy::prelude::*;

// states
use crate::states::GameState;

// feature files
pub mod components;
pub mod systems;

// feature plugin

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), systems::setup_player)
            .add_systems(
                Update,
                (systems::player_movement, systems::update_camera)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), systems::cleanup_player);
    }
}
