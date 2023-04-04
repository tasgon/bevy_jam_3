use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

mod character;
mod components;
mod input;
mod light;
mod physics;

#[derive(Component)]
pub struct MainCamera;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(LdtkPlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // Required to prevent race conditions between bevy_ecs_ldtk's and bevy_rapier's systems
        .configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, 0.0),
            ..Default::default()
        })
        .add_plugin(input::InputPlugin)
        .add_plugin(character::CharacterPlugin)
        .add_plugin(light::LightPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .add_system(physics::spawn_wall_collision)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/01.ldtk"),
        ..Default::default()
    });
}
