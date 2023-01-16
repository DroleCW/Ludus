use bevy::{prelude::*, window::WindowResized};

use super::{button, layout};

#[derive(Resource)]
pub struct MenuData {
    menu_buttons: Entity,
    background: Entity,
}
#[derive(Component)]
pub struct ButtonBox {
    margin_top: f32,
    margin_left: f32,
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_buttons = commands
        .spawn((
            layout::button_box(),
            ButtonBox {
                margin_top: 360.0,
                margin_left: 80.0,
            },
        ))
        .with_children(|parent| button::transparent_button(parent, &asset_server, "Join game"))
        .with_children(|parent| button::transparent_button(parent, &asset_server, "Settings"))
        .with_children(|parent| button::transparent_button(parent, &asset_server, "Exit"))
        .id();
    let background = commands
        .spawn(SpriteBundle {
            texture: asset_server.load("textures/main-menu.png"),
            transform: Transform {
                scale: (1.0, 1.0, 0.0).into(),
                ..default()
            },
            ..default()
        })
        .id();
    commands.insert_resource(MenuData {
        menu_buttons,
        background,
    });
}

pub fn window_resized_event(
    mut events: EventReader<WindowResized>,
    mut background: Query<(&mut Transform, &mut Handle<Image>)>,
    mut layout: Query<(&ButtonBox, &mut Style)>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.primary_mut();
    for event in events.iter() {
        let width_scale = event.width / 1280.0;
        let height_scale = event.height / 720.0;

        for (mut transform, _) in &mut background {
            transform.scale = (width_scale, height_scale, 1.0).into();
        }
        for (button_box, mut style) in &mut layout {
            style.margin.top = Val::Px(button_box.margin_top * (event.height / 720.0));
            style.margin.left = Val::Px(button_box.margin_left * (event.width / 1280.0));
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.menu_buttons).despawn_recursive();
    commands.entity(menu_data.background).despawn_recursive();
}
