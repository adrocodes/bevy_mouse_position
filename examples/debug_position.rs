use bevy::prelude::*;
use bevy_mouse_position::{debug_mouse_position, MousePositionPlugin};

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MousePositionPlugin)
        .add_startup_system(setup_camera)
        .add_system(debug_mouse_position)
        .run();
}
