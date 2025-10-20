// bevy
use bevy::prelude::*;

// components
use super::components::Ground;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Create infinite-looking ground plane (very large)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(500.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.2), // Green color
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Ground,
    ));

    // Add grid lines to help visualize movement
    let grid_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.6, 0.15), // Darker green for grid lines
        perceptual_roughness: 0.9,
        ..default()
    });

    // Create grid of small cubes as reference points
    for x in -10..=10 {
        for z in -10..=10 {
            if x % 5 == 0 && z % 5 == 0 {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.2, 0.1, 0.2))),
                    MeshMaterial3d(grid_material.clone()),
                    Transform::from_xyz(x as f32 * 2.0, 0.05, z as f32 * 2.0),
                    Ground,
                ));
            }
        }
    }
}

pub fn cleanup_world(mut commands: Commands, query: Query<Entity, With<Ground>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
