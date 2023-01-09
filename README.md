# Bevy Mouse Position

This plugin simple wraps the [Cookbook](https://bevy-cheatbook.github.io/cookbook/cursor2world.html) example to do the same thing. You can easily just copy and paste that example to your project if you wanted to.

---

## Install - cargo.toml

```toml
[dependencies]
bevy_mouse_position = { git = "https://github.com/adrocodes/bevy_mouse_position" }
```

## Usage

```rust
use bevy::prelude::*;
use bevy_mouse_position::{MousePositionPlugin, MousePosition};

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn use_mouse_position(position: Res<MousePosition>) {
    // position.0 - Vec2

    let x = position.0.x;
    let y = position.0.y;

    // Do something with x & y
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MousePositionPlugin)
        .add_startup_system(setup_camera)
        .add_system(use_mouse_position)
        .run();
}
```
