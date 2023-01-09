use bevy::{prelude::*, render::camera::RenderTarget};

pub struct MousePositionPlugin;

#[derive(Default, Debug, Resource, Copy, Clone)]
pub struct WorldPosition(pub Vec2);

#[derive(Default, Debug, Resource, Clone, Copy)]
pub struct CursorPositionUi(pub Vec2);

#[derive(Default, Debug, Resource, Clone, Copy)]
pub struct CursorPositionScreen(pub Vec2);

#[derive(Default, Debug, Resource)]
pub struct MousePosition {
    world: WorldPosition,
    cursor_ui: CursorPositionUi,
    cursor_screen: CursorPositionScreen,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum MousePositionSystems {
    Track,
}

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldPosition::default())
            .insert_resource(CursorPositionUi::default())
            .insert_resource(CursorPositionScreen::default())
            .insert_resource(MousePosition::default())
            .add_system(track_mouse_position.label(MousePositionSystems::Track));
    }
}

pub fn debug_world_position(position: Res<WorldPosition>) {
    println!("WORLD - ({}, {})", position.0.x, position.0.y);
}

pub fn debug_cursor_ui_position(position: Res<CursorPositionUi>) {
    println!("CURSOR UI - ({}, {})", position.0.x, position.0.y);
}

pub fn debug_cursor_screen_position(position: Res<CursorPositionScreen>) {
    println!("CURSOR SCREEN - ({}, {})", position.0.x, position.0.y);
}

pub fn debug_mouse_position(position: Res<MousePosition>) {
    println!("MOUSE POSITION - {:?}", position);
}

fn track_mouse_position(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform)>,
    mut res_world_position: ResMut<WorldPosition>,
    mut res_ui_position: ResMut<CursorPositionUi>,
    mut res_screen_position: ResMut<CursorPositionScreen>,
    mut res_mouse_position: ResMut<MousePosition>,
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

        res_world_position.0 = world_pos;
        res_ui_position.0 = ui_pos.abs();
        res_screen_position.0 = screen_pos;

        res_mouse_position.world = *res_world_position;
        res_mouse_position.cursor_ui = *res_ui_position;
        res_mouse_position.cursor_screen = *res_screen_position;
    }
}
