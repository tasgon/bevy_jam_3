use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::input::Cursor;

#[derive(Default, Component)]
pub struct Light {
    pub radius: f32,
}

#[derive(Default, Bundle)]
pub struct LightBundle {
    light: Light,
    spatial: SpatialBundle,
}

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_light)
            .add_system(spawn_light_data)
            .add_system(track_cursor.after(crate::input::cursor_system));
    }
}

fn spawn_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Light { radius: 64.0f32 })
        .with_children(|cmd| {
            cmd.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::new(1.0))).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE.with_a(0.5))),
                ..Default::default()
            });
        });
}

fn spawn_light_data(mut commands: Commands, lights: Query<Entity, Added<Light>>) {
    for entity in &lights {
        commands.entity(entity).insert(SpatialBundle::default());
    }
}

fn track_cursor(mut lights: Query<(&mut Transform, &Light)>, cursor: Res<Cursor>) {
    let Ok((mut transform, light)) = lights.get_single_mut() else { return };
    *transform =
        Transform::from_scale(Vec3::splat(light.radius)).with_translation(cursor.extend(100.0));
}
