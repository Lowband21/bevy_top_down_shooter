use crate::Animator;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMovement {
    pub speed: f32,
}

pub fn move_player(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerMovement, &mut Transform, &mut Animator)>,
) {
    for (player_movement, mut transform, mut animator) in query.iter_mut() {
        let mut is_moving = false;

        if keys.pressed(KeyCode::W) {
            transform.translation.y += player_movement.speed * time.delta_seconds();
            is_moving = true;
        }
        if keys.pressed(KeyCode::A) {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            transform.translation.x -= player_movement.speed * time.delta_seconds();
            is_moving = true;
        }
        if keys.pressed(KeyCode::S) {
            transform.translation.y -= player_movement.speed * time.delta_seconds();
            is_moving = true;
        }
        if keys.pressed(KeyCode::D) {
            transform.rotation = Quat::default();
            transform.translation.x += player_movement.speed * time.delta_seconds();
            is_moving = true;
        }

        // Update the animation state based on movement status
        if is_moving {
            animator.current_animation = "Run".to_string();
        } else {
            animator.current_animation = "Idle".to_string();
        }
    }
}
