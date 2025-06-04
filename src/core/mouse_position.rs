use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(MousePosition(Vec2::ZERO));
    app.add_systems(
        Update,
        update_cursor_world_position.in_set(UpdateSystems::SyncEarly),
    );
}

/// The mouse position in world coordinates.
#[derive(Resource)]
pub struct MousePosition(pub Vec2);

/// Updates the resource containing the cursor's position in world coordinates
fn update_cursor_world_position(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let (camera, camera_transform) = r!(camera_query.single());
    let cursor_position = r!(r!(window_query.single()).cursor_position());

    mouse_position.0 = r!(camera.viewport_to_world_2d(camera_transform, cursor_position));
}
