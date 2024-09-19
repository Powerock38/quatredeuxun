use avian3d::prelude::*;
use bevy::prelude::*;

use crate::table::TABLE_RADIUS;

pub const PLAYER_POSITION: Vec3 = Vec3::new(0.0, TABLE_RADIUS * 2.5, TABLE_RADIUS * 1.0);

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        crate::flycam::FlyCam,
        Camera3dBundle {
            transform: Transform::from_translation(PLAYER_POSITION).looking_at(Vec3::ZERO, Dir3::Y),
            ..default()
        },
    ));
}

pub fn click_spawns_raycast(
    mut commands: Commands,
    button_input: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let cursor_position = if button_input.just_pressed(MouseButton::Left) {
        windows.single().cursor_position()
    } else {
        touches
            .iter_just_pressed()
            .next()
            .map(|touch| touch.position())
    };

    let Some(cursor_position) = cursor_position else {
        return;
    };

    let (camera, camera_transform) = camera_query.single();

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    commands.spawn(RayCaster::from_ray(ray));
}
