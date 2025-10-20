use bevy::prelude::*;
use super::JointType;

/// Spawns the head with eyes
pub fn spawn_head(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    body_material: &Handle<StandardMaterial>,
    eye_material: &Handle<StandardMaterial>,
) {
    // Head
    parent.spawn((
        Mesh3d(meshes.add(Sphere::new(0.25))),
        MeshMaterial3d(body_material.clone()),
        Transform::from_xyz(0.0, 1.1, 0.0),
        JointType::Head,
    )).with_children(|head_parent| {
        // Left Eye - facing forward (negative Z in Bevy)
        head_parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.06))),
            MeshMaterial3d(eye_material.clone()),
            Transform::from_xyz(-0.08, 0.05, -0.22),
        ));

        // Right Eye
        head_parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.06))),
            MeshMaterial3d(eye_material.clone()),
            Transform::from_xyz(0.08, 0.05, -0.22),
        ));
    });
}
