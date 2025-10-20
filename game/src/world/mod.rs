// bevy
use bevy::prelude::*;

// states
use crate::states::GameState;

// feature files
pub mod components;
pub mod systems;

// feature plugin

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), systems::setup_world)
            .add_systems(OnExit(GameState::InGame), systems::cleanup_world);
    }
}
