#[path = "../ui/transparent-button.rs"]
mod button;
#[path = "../../settings/constants.rs"]
mod constants;
use super::state_machine::AppState;
use bevy::prelude::*;
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
#[derive(Resource)]
pub struct MenuData {
    button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_entity = commands
        .spawn(NodeBundle {
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
        })
        .with_children(|parent| button::transparent_button(parent, &asset_server, "Join game"))
        .with_children(|parent| button::transparent_button(parent, &asset_server, "Settings"))
        .with_children(|parent| button::transparent_button(parent, &asset_server, "Exit"))
        .id();
    commands.insert_resource(MenuData { button_entity });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/main-menu.png"),
        ..default()
    });
}

pub fn menu(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}
