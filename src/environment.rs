use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("TopDownExample.ldtk"),
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        ..Default::default()
    });
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
