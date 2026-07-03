use bevy::window::{Window, WindowPlugin, WindowResolution};

pub fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            mode: bevy::window::WindowMode::Windowed,
            resolution: WindowResolution::new(800, 800),
            resizable: true,
            ..Default::default()
        }),
        ..Default::default()
    }
}
