// bevy
use bevy::prelude::*;

use super::components::Player;

/// Spawns a simple humanoid character made of primitive shapes
pub fn spawn_shark_dino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) -> Entity {
    // Simple color for now
    let body_color = Color::srgb(0.7, 0.7, 0.7); // Light gray
    let eye_color = Color::srgb(0.1, 0.1, 0.1); // Dark gray/black

    // Materials
    let body_material = materials.add(StandardMaterial {
        base_color: body_color,
        perceptual_roughness: 0.6,
        ..default()
    });

    let eye_material = materials.add(StandardMaterial {
        base_color: eye_color,
        perceptual_roughness: 0.1,
        ..default()
    });

    // Main body container
    let player_entity = commands.spawn((
        Transform::from_translation(position),
        Player,
    )).id();

    // Build a simple humanoid character
    // Note: Adjusted positions so feet are at y=0 (ground level)
    // Eyes face FORWARD in Bevy's coordinate system (NEGATIVE Z)
    commands.entity(player_entity).with_children(|parent| {
        // Head (sphere)
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.25))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.0, 1.1, 0.0), // Head at top
        ));

        // Left Eye - pointing in NEGATIVE Z (Bevy's forward direction)
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.06))),
            MeshMaterial3d(eye_material.clone()),
            Transform::from_xyz(-0.08, 1.15, -0.22), // Left side of head, NEGATIVE Z = forward
        ));

        // Right Eye - pointing in NEGATIVE Z (Bevy's forward direction)
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.06))),
            MeshMaterial3d(eye_material.clone()),
            Transform::from_xyz(0.08, 1.15, -0.22), // Right side of head, NEGATIVE Z = forward
        ));

        // Body/Torso (tall capsule)
        parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.15, 0.4))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.0, 0.7, 0.0), // Torso in middle
        ));

        // Left Arm - properly attached to shoulder
        parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.05, 0.25))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(-0.2, 0.8, 0.0)
                .with_rotation(Quat::from_rotation_z(0.5)), // Angled down from shoulder
        ));

        // Right Arm - properly attached to shoulder
        parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.05, 0.25))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.2, 0.8, 0.0)
                .with_rotation(Quat::from_rotation_z(-0.5)), // Angled down from shoulder
        ));

        // Left Leg (thin capsule) - positioned so bottom touches ground
        parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.06, 0.35))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(-0.1, 0.23, 0.0), // Legs start from lower torso
        ));

        // Right Leg (thin capsule) - positioned so bottom touches ground
        parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.06, 0.35))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.1, 0.23, 0.0), // Legs start from lower torso
        ));
    });

    player_entity
}
