// bevy
use bevy::prelude::*;

/// Main game state
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    StartScreen,
    InGame,
}
