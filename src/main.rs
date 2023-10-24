use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashMap;

pub mod animation;
pub mod player;
use crate::animation::*;

pub mod environment;
use crate::environment::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<Player>("Player")
        .register_ldtk_int_cell::<WallBundle>(1)
        .add_systems(
            Update,
            animation::switch_animation_state.before(animation::animate_sprite),
        )
        .add_systems(Update, animation::animate_sprite)
        .add_systems(Update, player::move_player)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_level)
        .run();
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load the textures and create texture atlases as before.
    let idle_texture_handle = asset_server.load("Heroes/Knight/Idle/Idle-Sheet.png");
    let idle_texture_atlas = TextureAtlas::from_grid(
        idle_texture_handle,
        Vec2::new(32.0, 32.0),
        4,
        1,
        Some(Vec2::new(0., 0.)),
        None,
    );
    let idle_texture_atlas_handle = texture_atlases.add(idle_texture_atlas);

    let run_texture_handle = asset_server.load("Heroes/Knight/Run/Run-Sheet.png");
    let run_texture_atlas = TextureAtlas::from_grid(
        run_texture_handle,
        Vec2::new(32.0, 32.0),
        6,
        1,
        Some(Vec2::new(32., 0.)),
        Some(Vec2::new(16., 32.)),
    );
    let run_texture_atlas_handle = texture_atlases.add(run_texture_atlas);

    // Create the camera entity as before.
    commands.spawn(Camera2dBundle::default());

    // Create a player entity using the new `Player` bundle.
    commands.spawn(Player {
        animator: Animator {
            animations: create_player_anim_hashmap(), // You would define this function elsewhere to return the HashMap used previously.
            current_animation: "Idle".to_string(),
            run_sprite: Some(run_texture_atlas_handle.clone()),
            idle_sprite: Some(idle_texture_atlas_handle.clone()),
            timer: 0.0,
            current_frame: 0,
        },
        player_movement: player::PlayerMovement { speed: 400.0 },
        sprite_bundle: SpriteSheetBundle {
            texture_atlas: idle_texture_atlas_handle,
            ..Default::default()
        },
    });
}

#[derive(Bundle, LdtkEntity)]
pub struct Player {
    animator: Animator,
    player_movement: player::PlayerMovement,
    sprite_bundle: SpriteSheetBundle,
}

impl Default for Player {
    fn default() -> Player {
        Self {
            animator: Animator {
                animations: create_player_anim_hashmap(),
                current_animation: "Idle".to_string(),
                run_sprite: None,
                idle_sprite: None,
                current_frame: 0,
                timer: 0.0,
            },
            player_movement: player::PlayerMovement { speed: 400.0 },
            sprite_bundle: SpriteSheetBundle::default(),
        }
    }
}
pub fn create_player_anim_hashmap() -> HashMap<String, animation::Animation> {
    let mut hash_map = HashMap::new();

    hash_map.insert(
        "Run".to_string(),
        animation::Animation {
            start: 0,
            end: 5,
            looping: true,
            cooldown: 0.1,
        },
    );

    hash_map.insert(
        "Idle".to_string(),
        animation::Animation {
            start: 0,
            end: 3,
            looping: true,
            cooldown: 0.1,
        },
    );

    return hash_map;
}
