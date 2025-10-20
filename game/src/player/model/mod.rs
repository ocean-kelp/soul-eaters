// Body part modules
mod body;
mod limbs;
mod head;

// Re-export the main spawn function
pub use body::spawn_shark_dino;

// Joint component for skeletal animation later
use bevy::prelude::*;

/// Represents a joint in the character's skeleton
/// This will be useful for animations later
#[derive(Component)]
#[allow(dead_code)]
pub struct Joint {
    pub name: String,
    pub parent: Option<Entity>,
}

/// Different joint types in our character
#[derive(Component, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum JointType {
    // Core
    Root,
    Spine,
    Neck,
    Head,
    
    // Arms
    LeftShoulder,
    LeftElbow,
    LeftWrist,
    RightShoulder,
    RightElbow,
    RightWrist,
    
    // Legs
    LeftHip,
    LeftKnee,
    LeftAnkle,
    RightHip,
    RightKnee,
    RightAnkle,
    
    // Claws (fingers/toes)
    Claw,
}
