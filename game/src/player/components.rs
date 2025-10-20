// bevy
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera {
    pub distance: f32,
    pub height: f32,
    pub angle_horizontal: f32, // Left/right rotation around player
    pub angle_vertical: f32,   // Up/down angle (pitch)
    pub sensitivity: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            distance: 6.0,
            height: 3.0,
            angle_horizontal: 0.0,
            angle_vertical: 0.3, // Slight downward angle
            sensitivity: 0.003,
        }
    }
}

#[derive(Component)]
pub struct Velocity {
    pub linear: Vec3,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            linear: Vec3::ZERO,
        }
    }
}
