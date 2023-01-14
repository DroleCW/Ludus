use bevy::prelude::*;

use super::{button, layout};

#[derive(Resource)]
pub struct MenuData {
    menu_buttons: Entity,
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_buttons = commands
        .spawn(layout::button_box())
        .with_children(|parent| button::transparent_button(parent, &asset_server, "Join game"))
        .with_children(|parent| button::transparent_button(parent, &asset_server, "Settings"))
        .with_children(|parent| button::transparent_button(parent, &asset_server, "Exit"))
        .id();
    commands.insert_resource(MenuData { menu_buttons });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/main-menu.png"),
        ..default()
    });
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.menu_buttons).despawn_recursive();
}
