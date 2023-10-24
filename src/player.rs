use crate::Animator;
use bevy::math::Vec3;
use bevy::prelude::*; // Import Vec3 for the movement vector

#[derive(Component)]
pub struct PlayerMovement {
    pub speed: f32,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        PlayerMovement { speed: 400.0 }
    }
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerMovement, &mut Transform, &mut Animator)>,
) {
    for (player_movement, mut transform, mut animator) in query.iter_mut() {
        let mut movement = Vec3::ZERO; // Create a vector to accumulate movement directions

        if keys.pressed(KeyCode::W) {
            movement += Vec3::new(0.0, 1.0, 0.0); // Up
        }
        if keys.pressed(KeyCode::S) {
            movement += Vec3::new(0.0, -1.0, 0.0); // Down
        }
        if keys.pressed(KeyCode::A) {
            movement += Vec3::new(-1.0, 0.0, 0.0); // Left
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }
        if keys.pressed(KeyCode::D) {
            movement += Vec3::new(1.0, 0.0, 0.0); // Right
            transform.rotation = Quat::default();
        }

        // Check if there's any intended movement
        if movement != Vec3::ZERO {
            // Normalize the movement vector so the player moves consistently in all directions
            movement = movement.normalize();

            // Apply the movement to the player's position
            transform.translation += movement * player_movement.speed * time.delta_seconds();
        }

        // Update the animation state based on the actual movement
        if movement != Vec3::ZERO {
            animator.current_animation = "Run".to_string();
        } else {
            animator.current_animation = "Idle".to_string();
        }
    }
}
