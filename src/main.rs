use bevy::prelude::*;
use std::collections::HashMap;

pub mod animation;
pub mod player;
use crate::animation::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Update, animation::animate_sprite)
        .add_systems(Update, player::move_player)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct Idle;
#[derive(Component)]
pub struct Run;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut idle_texture_handle = asset_server.load("Heroes/Knight/Idle/Idle-Sheet.png");
    let mut idle_texture_atlas = TextureAtlas::from_grid(
        idle_texture_handle,
        Vec2::new(32.0, 32.0),
        4,
        1,
        Some(Vec2::new(0., 0.)),
        None,
    );
    let mut idle_texture_atlas_handle = texture_atlases.add(idle_texture_atlas);

    let mut run_texture_handle = asset_server.load("Heroes/Knight/Run/Run-Sheet.png");
    let mut run_texture_atlas = TextureAtlas::from_grid(
        run_texture_handle,
        Vec2::new(32.0, 32.0),
        6,
        1,
        Some(Vec2::new(0., 0.)),
        None,
    );
    let mut run_texture_atlas_handle = texture_atlases.add(run_texture_atlas);

    // Create the animation hashmap containing our animations with their corresponding details.
    let mut animations = HashMap::new();
    animations.insert(
        "Idle".to_string(),
        Animation {
            start: 0, // Frame index starts at 0.
            end: 3,
            cooldown: 0.1,
            looping: true,
        },
    );
    animations.insert(
        "Run".to_string(),
        Animation {
            start: 0,
            end: 5,
            cooldown: 0.05,
            looping: true,
        },
    );

    // Spawn our main entity (e.g., the player) with the `Animator` component and the animations.
    commands.spawn(Camera2dBundle::default());
    let player_entity = commands
        .spawn(SpriteSheetBundle {
            texture_atlas: idle_texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(2.0)), // Scale as needed.
            ..Default::default()
        })
        .insert(Animator {
            animations,
            current_animation: "Idle".to_string(),
            active_entity: None, // Initially, no animation is playing.
            timer: 0.0,
            current_frame: 0,
        })
        // Add any other components your player entity needs, e.g., for movement.
        .insert(player::PlayerMovement { speed: 400.0 })
        .id();

    // Spawn child entities for each animation type.
    // These will be made visible/invisible depending on the active animation.
    commands.entity(player_entity).with_children(|parent| {
        parent
            .spawn(SpriteSheetBundle {
                texture_atlas: idle_texture_atlas_handle,
                ..Default::default()
            })
            .insert(Idle); // This `Idle` marker component can be used for queries.

        parent
            .spawn(SpriteSheetBundle {
                texture_atlas: run_texture_atlas_handle,
                ..Default::default()
            })
            .insert(Run); // This `Run` marker component can be used for queries.
    });
}
pub fn create_player_anim_hashmap() -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();

    hash_map.insert(
        "Walk".to_string(),
        animation::Animation {
            start: 1,
            end: 3,
            looping: true,
            cooldown: 0.1,
        },
    );

    hash_map.insert(
        "Idle".to_string(),
        animation::Animation {
            start: 1,
            end: 4,
            looping: true,
            cooldown: 0.1,
        },
    );

    return hash_map;
}
