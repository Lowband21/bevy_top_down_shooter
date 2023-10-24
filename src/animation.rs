use crate::{Idle, Run};
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
    pub active_entity: Option<Entity>,
    pub timer: f32,
}

impl Animator {
    pub fn new(animations: HashMap<String, Animation>) -> Self {
        Self {
            animations,
            current_animation: "Idle".to_string(), // default animation
            active_entity: None,
            timer: 0.0,
            current_frame: 0,
        }
    }
}

// This system updates the frames of the animated sprite and changes the visible animation entity.
// Correct the function signature. We don't need separate queries for Idle and Run entities.

pub fn animate_sprite(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &mut Animator, &Children)>,
    mut run_sprite_query: Query<
        (&mut TextureAtlasSprite, &Handle<TextureAtlas>),
        (With<Run>, Without<Idle>),
    >,
    mut idle_sprite_query: Query<
        (&mut TextureAtlasSprite, &Handle<TextureAtlas>),
        (With<Idle>, Without<Run>),
    >,
) {
    for (entity, mut animator, children) in query.iter_mut() {
        let current_animation = animator.current_animation.clone();

        let animation = if let Some(animation) = animator.animations.get(&current_animation) {
            animation.clone()
        } else {
            continue;
        };

        animator.timer -= time.delta().as_secs_f32();

        if animator.timer <= 0.0 {
            animator.timer = animation.cooldown;

            let num_frames = (animation.end - animation.start) + 1;
            let next_frame = (animator.current_frame + 1) % num_frames;
            animator.current_frame = if next_frame < animation.start || next_frame > animation.end {
                animation.start
            } else {
                next_frame
            };

            // Update the sprite for each child based on the current animation.
            for &child in children.iter() {
                let atlas_index = animation.start + animator.current_frame;

                // Depending on the current animation, we update the corresponding sprite
                // and make the other one invisible if it's currently visible.
                if current_animation == "Run" {
                    if let Ok((mut sprite, atlas_handle)) = run_sprite_query.get_mut(child) {
                        if let Some(atlas) = texture_atlases.get(atlas_handle) {
                            sprite.index = atlas_index;
                            commands.entity(child).insert(Visible); // Make the entity visible.
                        }
                    }

                    // We need to ensure the other texture is not visible.
                    for (mut sprite, _) in idle_sprite_query.iter_mut() {
                        commands.entity(entity).remove::<Visibility>(); // Make the entity invisible.
                    }
                } else if current_animation == "Idle" {
                    if let Ok((mut sprite, atlas_handle)) = idle_sprite_query.get_mut(child) {
                        if let Some(atlas) = texture_atlases.get(atlas_handle) {
                            sprite.index = atlas_index;
                            commands.entity(child).insert(Visibility::Visible); // Make the entity visible.
                        }
                    }

                    // We need to ensure the other texture is not visible.
                    for (mut sprite, _) in run_sprite_query.iter_mut() {
                        commands.entity(entity).remove::<Visibility>(); // Make the entity invisible.
                    }
                }
                // You can add more conditions here for other animations.
            }
        }
    }
}
