use bevy::prelude::*;

use super::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn button_box() -> NodeBundle {
    NodeBundle {
        style: Style {
            // center button
            size: Size::new(Val::Percent(25.0), Val::Percent(50.0)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            flex_direction: FlexDirection::Column,
            margin: UiRect {
                top: Val::Px((WINDOW_HEIGHT / 2) as f32),
                left: Val::Px((WINDOW_WIDTH / 16) as f32),
                ..default()
            },
            ..default()
        },
        ..default()
    }
}
