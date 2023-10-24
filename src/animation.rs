use crate::create_player_anim_hashmap;
use bevy::prelude::*;
use bevy::render::view::Visibility::Visible;
use std::collections::HashMap;

// Animation details including frames and playback behavior.
#[derive(Clone, Copy)]
pub struct Animation {
    pub cooldown: f32,
    pub start: usize,
    pub end: usize,
    pub looping: bool,
}

// Animator component for entities that should display animated sprites.
#[derive(Clone, Component)]
pub struct Animator {
    pub animations: HashMap<String, Animation>,
    pub current_animation: String,
    pub current_frame: usize,
    pub idle_sprite: Option<Handle<TextureAtlas>>,
    pub run_sprite: Option<Handle<TextureAtlas>>,
    pub timer: f32,
}

impl Default for Animator {
    fn default() -> Self {
        Animator {
            animations: create_player_anim_hashmap(),
            current_animation: "Idle".to_string(),
            idle_sprite: None,
            run_sprite: None,
            current_frame: 0,
            timer: 0.0,
        }
    }
}
impl Animator {
    pub fn new(animations: HashMap<String, Animation>) -> Self {
        Self {
            animations,
            current_animation: "Idle".to_string(), // default animation
            idle_sprite: None,
            run_sprite: None,
            timer: 0.0,
            current_frame: 0,
        }
    }
}

// This system updates the frames of the animated sprite and changes the visible animation entity.
// Correct the function signature. We don't need separate queries for Idle and Run entities.

pub fn switch_animation_state(
    mut query: Query<(
        &mut Animator,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (animator, mut atlas_handle, mut sprite) in query.iter_mut() {
        let current_animation = animator.current_animation.clone();

        if animator.is_changed() {
            match current_animation.as_str() {
                "Idle" => match animator.idle_sprite.clone() {
                    Some(idle) => {
                        *atlas_handle = idle;
                        if sprite.index > 3 {
                            sprite.index = 0;
                        }
                    }
                    None => continue,
                },
                "Run" => match animator.run_sprite.clone() {
                    Some(run) => *atlas_handle = run,
                    None => continue,
                },
                &_ => panic!("Invalid Current Animation"),
            }
        }
    }
}

pub fn animate_sprite(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Animator, &mut TextureAtlasSprite)>,
) {
    // Iterate through all entities with Animator component.
    for (mut animator, mut sprite) in query.iter_mut() {
        let current_animation = animator.current_animation.clone();

        let animations = &animator.animations.clone();
        // Get the current animation. If it's None, we skip this entity.
        let animation = animations.get(&current_animation).unwrap();

        // Process cooldowns and frames based on time.
        animator.timer -= time.delta().as_secs_f32();

        if animator.timer <= 0.0 {
            animator.timer = animation.cooldown; // reset after reaching 0

            let num_frames = (animation.end - animation.start) + 1;
            println!(
                "Current animation: ({}), num frames: {}",
                current_animation, num_frames
            );
            animator.current_frame = (sprite.index + 1) % num_frames;

            // If we have reached the end of the animation, loop from the start.
            if animator.current_frame < animation.start || animator.current_frame > animation.end {
                animator.current_frame = animation.start;
            }

            // Set the new sprite frame.
            let atlas_index = animation.start + animator.current_frame; // calculate the new frame index
            sprite.index = atlas_index; // apply the new frame

            // We don't make entities invisible but you can add additional logic here to handle visibility if needed.
            // For example, based on states or other conditions, you might want to remove/add entities, components, etc.
        }
    }
}
