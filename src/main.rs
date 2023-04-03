use bevy::{prelude::{App, Camera2dBundle, Commands, Res, AssetServer}, DefaultPlugins};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/01.ldtk"),
        ..Default::default()
    });
}

