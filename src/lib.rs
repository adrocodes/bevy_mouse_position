use bevy::{prelude::*, render::camera::RenderTarget};

pub struct MousePositionPlugin;

#[derive(Default, Debug, Resource)]
pub struct MousePosition(pub Vec2);

#[derive(Default, Debug, Resource)]
pub struct UiMousePosition(pub Vec2);

pub const MOUSE_POSITION_TRACKING_LABEL: &str = "track_mouse_position";

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition::default())
            .insert_resource(UiMousePosition::default())
            .add_system(track_mouse_position.label(MOUSE_POSITION_TRACKING_LABEL));
    }
}

pub fn debug_mouse_position(position: Res<MousePosition>) {
    println!("x: {}, y: {}", position.0.x, position.0.y);
}

pub fn debug_ui_mouse_position(position: Res<UiMousePosition>) {
    println!("x: {}, y: {}", position.0.x, position.0.y);
}

fn track_mouse_position(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform)>,
    mut position: ResMut<MousePosition>,
    mut ui_position: ResMut<UiMousePosition>,
) {
    let (camera, camera_transform) = query.single();

    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        let world_pos: Vec2 = world_pos.truncate();
        let ui_pos = screen_pos - Vec2::new(0., window_size.y);

        position.0 = world_pos;
        ui_position.0 = ui_pos.abs();
    }
}
