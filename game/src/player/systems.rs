// bevy
use bevy::prelude::*;

// components
use super::components::{Player, PlayerCamera, Velocity};

const PLAYER_SPEED: f32 = 5.0;
const JUMP_FORCE: f32 = 8.0;
const GRAVITY: f32 = -20.0;
const GROUND_LEVEL: f32 = 0.5; // Half the cube height

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the player cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2), // Red cube
            ..default()
        })),
        Transform::from_xyz(0.0, GROUND_LEVEL, 0.0),
        Player,
        Velocity::default(),
    ));

    // Spawn third-person camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        PlayerCamera,
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in &mut player_query {
        let mut direction = Vec3::ZERO;

        // Get input direction - WASD
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        // Arrow keys
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        // Normalize diagonal movement
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // Apply horizontal movement
        velocity.linear.x = direction.x * PLAYER_SPEED;
        velocity.linear.z = direction.z * PLAYER_SPEED;

        // Jump logic
        let is_on_ground = transform.translation.y <= GROUND_LEVEL;
        if is_on_ground && keyboard_input.pressed(KeyCode::Space) {
            velocity.linear.y = JUMP_FORCE;
        }

        // Apply gravity
        velocity.linear.y += GRAVITY * time.delta_secs();

        // Update position
        transform.translation += velocity.linear * time.delta_secs();

        // Keep player above ground
        if transform.translation.y < GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.linear.y = 0.0;
        }
    }
}

pub fn update_camera(
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            // Camera follows player with offset
            let camera_offset = Vec3::new(0.0, 5.0, 10.0);
            let target_position = player_transform.translation + camera_offset;
            
            camera_transform.translation = target_position;
            camera_transform.look_at(player_transform.translation + Vec3::Y, Vec3::Y);
        }
    }
}

pub fn cleanup_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<Entity, With<PlayerCamera>>,
) {
    for entity in &player_query {
        commands.entity(entity).despawn();
    }
    for entity in &camera_query {
        commands.entity(entity).despawn();
    }
}
