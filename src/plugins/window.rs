use bevy::window::{Window, WindowPlugin, WindowResolution};

pub fn window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            mode: bevy::window::WindowMode::Windowed,
            resolution: WindowResolution::new(1280, 720),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }
}
