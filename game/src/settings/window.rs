use super::constants;
use bevy::{prelude::*, window::PresentMode};
pub fn window_settings() -> bevy::prelude::WindowPlugin {
    WindowPlugin {
        window: WindowDescriptor {
            title: "Ludus".to_string(),
            width: constants::WINDOW_WIDTH as f32,
            height: constants::WINDOW_HEIGHT as f32,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    }
}
