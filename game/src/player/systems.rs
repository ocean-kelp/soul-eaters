// bevy
use bevy::prelude::*;

// components
use super::components::{Player, PlayerCamera, Velocity};
// model
use super::model::spawn_shark_dino;

const PLAYER_SPEED: f32 = 5.0;
const JUMP_FORCE: f32 = 8.0;
const GRAVITY: f32 = -20.0;
const GROUND_LEVEL: f32 = 0.0; // Feet on the ground

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the shark-dino player slightly above ground so they fall naturally
    let player_entity = spawn_shark_dino(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 5.0, 0.0), // Spawn 5 units above ground
    );

    // Add velocity component to the player
    commands.entity(player_entity).insert(Velocity::default());

    // Spawn third-person camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-4.0, 3.0, 6.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        PlayerCamera::default(),
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in &mut player_query {
        const ROTATION_SPEED: f32 = 3.0; // Radians per second
        let mut is_moving = false;

        // Rotation controls (left/right/down arrows)
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            // Rotate left (counter-clockwise)
            transform.rotate_y(ROTATION_SPEED * time.delta_secs());
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            // Rotate right (clockwise)
            transform.rotate_y(-ROTATION_SPEED * time.delta_secs());
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            // Rotate 180 degrees (turn around)
            transform.rotate_y(ROTATION_SPEED * time.delta_secs());
        }

        // Forward movement (up arrow only)
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            is_moving = true;
            // Move forward in the direction the player is facing
            let forward = transform.forward();
            let movement_direction = Vec3::new(forward.x, 0.0, forward.z).normalize();
            
            velocity.linear.x = movement_direction.x * PLAYER_SPEED;
            velocity.linear.z = movement_direction.z * PLAYER_SPEED;
        }

        // Stop horizontal movement if not pressing up arrow
        if !is_moving {
            velocity.linear.x = 0.0;
            velocity.linear.z = 0.0;
        }

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
    mut camera_query: Query<(&mut Transform, &mut PlayerCamera)>,
    mut mouse_motion: MessageReader<bevy::input::mouse::MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok((mut camera_transform, mut camera)) = camera_query.single_mut() {
            // Rotate camera when right mouse button is held
            if mouse_button.pressed(MouseButton::Right) {
                for event in mouse_motion.read() {
                    // Horizontal rotation (left/right)
                    camera.angle_horizontal -= event.delta.x * camera.sensitivity;
                    
                    // Vertical rotation (up/down)
                    camera.angle_vertical -= event.delta.y * camera.sensitivity;
                    
                    // Clamp vertical angle to prevent camera flipping
                    camera.angle_vertical = camera.angle_vertical.clamp(-1.5, 1.5);
                }
            } else {
                // Clear the mouse motion events if not using them
                mouse_motion.clear();
            }

            // Get player's forward direction
            let player_forward = player_transform.forward();
            
            // Calculate base backward direction (behind player)
            let backward_direction = Vec3::new(-player_forward.x, 0.0, -player_forward.z).normalize();
            
            // Apply horizontal rotation offset around the player
            let rotated_backward = Quat::from_rotation_y(camera.angle_horizontal) * backward_direction;
            
            // Calculate camera position with both horizontal and vertical angles
            let horizontal_distance = camera.distance * camera.angle_vertical.cos();
            let vertical_distance = camera.distance * camera.angle_vertical.sin();
            
            let camera_offset = Vec3::new(
                rotated_backward.x * horizontal_distance,
                camera.height + vertical_distance,
                rotated_backward.z * horizontal_distance,
            );
            
            let target_position = player_transform.translation + camera_offset;
            camera_transform.translation = target_position;
            
            // Look at player's center (slightly above ground)
            camera_transform.look_at(player_transform.translation + Vec3::Y * 0.5, Vec3::Y);
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
