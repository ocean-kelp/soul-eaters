use bevy::prelude::*;
use super::JointType;

/// Spawns an arm with shoulder, elbow, wrist, and 3-fingered claw
pub fn spawn_arm(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    body_material: &Handle<StandardMaterial>,
    claw_material: &Handle<StandardMaterial>,
    is_left: bool,
) {
    let side = if is_left { -1.0 } else { 1.0 };
    let rotation_dir = if is_left { 0.5 } else { -0.5 };
    
    // Shoulder joint position
    let shoulder_pos = Vec3::new(side * 0.2, 0.8, 0.0);
    
    // Shoulder (upper arm)
    parent.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.05, 0.15))),
        MeshMaterial3d(body_material.clone()),
        Transform::from_translation(shoulder_pos)
            .with_rotation(Quat::from_rotation_z(rotation_dir)),
        if is_left { JointType::LeftShoulder } else { JointType::RightShoulder },
    )).with_children(|shoulder| {
        // Elbow (forearm) - extends down from shoulder
        shoulder.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.04, 0.15))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.0, -0.15, 0.0),
            if is_left { JointType::LeftElbow } else { JointType::RightElbow },
        )).with_children(|elbow| {
            // Wrist/Hand base - the "palm"
            elbow.spawn((
                Mesh3d(meshes.add(Capsule3d::new(0.04, 0.08))),
                MeshMaterial3d(body_material.clone()),
                Transform::from_xyz(0.0, -0.15, 0.0), // Hand extends down from elbow
                if is_left { JointType::LeftWrist } else { JointType::RightWrist },
            )).with_children(|wrist| {
                // 3 Claws/Fingers extending from hand
                spawn_claw_fingers(wrist, meshes, claw_material, 0.1, true); // true = hand claws
            });
        });
    });
}

/// Spawns a leg with hip, knee, ankle, and 3-toed claw foot
pub fn spawn_leg(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    body_material: &Handle<StandardMaterial>,
    claw_material: &Handle<StandardMaterial>,
    is_left: bool,
) {
    let side = if is_left { -1.0 } else { 1.0 };
    
    // Hip joint position
    let hip_pos = Vec3::new(side * 0.1, 0.5, 0.0);
    
    // Hip (thigh)
    parent.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.06, 0.2))),
        MeshMaterial3d(body_material.clone()),
        Transform::from_translation(hip_pos),
        if is_left { JointType::LeftHip } else { JointType::RightHip },
    )).with_children(|hip| {
        // Knee (shin/calf)
        hip.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.055, 0.18))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.0, -0.2, 0.0),
            if is_left { JointType::LeftKnee } else { JointType::RightKnee },
        )).with_children(|knee| {
            // Ankle/Foot base (foot shape)
            knee.spawn((
                Mesh3d(meshes.add(Capsule3d::new(0.05, 0.12))),
                MeshMaterial3d(body_material.clone()),
                Transform::from_xyz(0.0, -0.18, 0.0), // Foot extends down
                if is_left { JointType::LeftAnkle } else { JointType::RightAnkle },
            )).with_children(|ankle| {
                // 3 Claws/Toes extending from foot
                spawn_claw_fingers(ankle, meshes, claw_material, 0.12, false); // false = foot claws
            });
        });
    });
}

/// Spawns 3 claw fingers/toes in a spread pattern
fn spawn_claw_fingers(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    claw_material: &Handle<StandardMaterial>,
    length: f32,
    is_hand: bool, // true for hands, false for feet
) {
    // Create 3 claws spread in a fan pattern
    let angles = [-0.5, 0.0, 0.5]; // Spread angles in radians
    
    for (i, &angle) in angles.iter().enumerate() {
        if is_hand {
            // Hand claws - cone pointing forward (negative Z) with horizontal fan spread
            parent.spawn((
                Mesh3d(meshes.add(Cone {
                    radius: 0.015,
                    height: length,
                })),
                MeshMaterial3d(claw_material.clone()),
                Transform::from_xyz(
                    (i as f32 - 1.0) * 0.04, // Horizontal spread
                    -0.04, // Slightly below hand
                    -0.05, // Forward (negative Z = forward in Bevy)
                )
                .with_rotation(
                    Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2) * // Point forward (cone now lies along Z axis)
                    Quat::from_rotation_y(angle) * // Spread horizontally around the forward axis
                    Quat::from_rotation_x(0.3) // Add downward tilt
                ),
                JointType::Claw,
            ));
        } else {
            // Foot claws - cone pointing forward (negative Z) with horizontal fan spread
            parent.spawn((
                Mesh3d(meshes.add(Cone {
                    radius: 0.018,
                    height: length,
                })),
                MeshMaterial3d(claw_material.clone()),
                Transform::from_xyz(
                    (i as f32 - 1.0) * 0.045, // Horizontal spread
                    -0.06, // Slightly below foot
                    -0.06, // Forward (negative Z = forward in Bevy)
                )
                .with_rotation(
                    Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2) * // Point forward (cone now lies along Z axis)
                    Quat::from_rotation_y(angle) * // Spread horizontally around the forward axis
                    Quat::from_rotation_x(0.4) // Add downward tilt
                ),
                JointType::Claw,
            ));
        }
    }
}
