use bevy::{prelude::*, window::{PrimaryWindow, WindowRef}, render::camera::RenderTarget};

/// Cursor position in world
#[derive(Resource, Deref, DerefMut, Default, Debug)]
pub struct Cursor(pub Vec2);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Cursor::default())
            .add_system(cursor_system);
    }
}

fn cursor_system(
    windows: Query<&Window>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
    mut cursor: ResMut<Cursor>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(WindowRef::Entity(id)) = camera.target {
        windows.get(id).unwrap()
    } else {
        primary_window.single()
    };

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor.0 = world_position;
    }
}