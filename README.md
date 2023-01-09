# Bevy Mouse Position

This plugin wraps the [Cookbook](https://bevy-cheatbook.github.io/cookbook/cursor2world.html) example to do the same thing and adds some extra stuff to it.

---

## Install - cargo.toml

```toml
[dependencies]
bevy_mouse_position = { git = "https://github.com/adrocodes/bevy_mouse_position" }
```

## Usage

```rust
use bevy::prelude::*;
use bevy_mouse_position::{MousePositionPlugin, WorldPosition};

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn use_mouse_position(position: Res<WorldPosition>) {
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

## Resources

The plugin updates 4 different resources.

1. `WorldPosition` - this is the World position of the cursor, you can use this as the entity transform.
2. `CursorPositionUi` - this is the position that can be used for UI coordinates. Top-Left corner of the screen will be 0,0.
3. `CursorPositionScreen` - this is the `window.cursor_position()` value. You can use this to do your own calculations. The Bottom-Left corner will be 0,0.
4. `MousePosition` - a struct that holds all the above 3 values. 

## Methods

The library exports 4 `debug` systems you can use to debug the different values of the resources.

- `debug_world_position` - prints the value of `WorldPosition`
- `debug_cursor_ui_position` - prints the value of `CursorPositionUi`
- `debug_cursor_screen_position` - prints the value of `CursorPositionScreen`
- `debug_mouse_position` - prints the value of `MousePosition`
