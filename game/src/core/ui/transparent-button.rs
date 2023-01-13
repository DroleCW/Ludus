use bevy::prelude::*;
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub fn transparent_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, text: &str) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(48.0)),
                // horizontally center child text
                justify_content: JustifyContent::FlexStart,
                // vertically center child text
                align_items: AlignItems::Center,
                margin: UiRect {
                    bottom: Val::Px((10) as f32),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts\\inter.ttf"),
                    font_size: 32.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
