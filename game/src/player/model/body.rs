use bevy::prelude::*;
use super::{Joint, JointType};
use super::head::spawn_head;
use super::limbs::{spawn_arm, spawn_leg};
use crate::player::components::Player;

/// Spawns the complete shark-dino character
pub fn spawn_shark_dino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) -> Entity {
    // Colors
    let body_color = Color::srgb(0.6, 0.7, 0.75); // Slightly blue-gray (shark-ish)
    let claw_color = Color::srgb(0.9, 0.9, 0.85); // Off-white for claws

    // Materials
    let body_material = materials.add(StandardMaterial {
        base_color: body_color,
        perceptual_roughness: 0.6,
        ..default()
    });

    let claw_material = materials.add(StandardMaterial {
        base_color: claw_color,
        perceptual_roughness: 0.3,
        ..default()
    });

    let eye_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.05, 0.05), // Very dark/black
        metallic: 0.8, // Metallic for reflections
        perceptual_roughness: 0.1, // Smooth and shiny
        reflectance: 0.9, // High reflectance
        ..default()
    });

    // Root entity - the character's base
    let root_entity = commands.spawn((
        Transform::from_translation(position),
        Player,
        Joint {
            name: "Root".to_string(),
            parent: None,
        },
        JointType::Root,
    )).id();

    commands.entity(root_entity).with_children(|parent| {
        // Spine/Torso
        let _spine = parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.15, 0.4))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.0, 0.7, 0.0),
            JointType::Spine,
        )).id();

        // Head (attached to top of spine)
        spawn_head(parent, meshes, &body_material, &eye_material);

        // Left Arm (from shoulder)
        spawn_arm(
            parent,
            meshes,
            &body_material,
            &claw_material,
            true, // is_left
        );

        // Right Arm (from shoulder)
        spawn_arm(
            parent,
            meshes,
            &body_material,
            &claw_material,
            false, // is_right
        );

        // Left Leg (from hip)
        spawn_leg(
            parent,
            meshes,
            &body_material,
            &claw_material,
            true, // is_left
        );

        // Right Leg (from hip)
        spawn_leg(
            parent,
            meshes,
            &body_material,
            &claw_material,
            false, // is_right
        );
    });

    root_entity
}
