use bevy::prelude::*;

use super::constants::colors;
use crate::connection::connection_resource::ConnectionRes;
use crate::connection::send::send;

use crate::core::states::state_machine::AppState;

pub fn handle_join_button(connection: &mut ConnectionRes) {
    println!("join game");
    let msg = b"{\"username\": \"diogodsg\", \"action\": \"join\", \"data\": \"aa\"}\n";

    send(connection, msg);
}

fn toogle_screen_mode(window: &mut Window) {
    match window.mode() {
        WindowMode::Fullscreen => window.set_mode(WindowMode::Windowed),
        WindowMode::Windowed => window.set_mode(WindowMode::Fullscreen),
        _ => window.set_mode(WindowMode::Windowed),
    }
}

pub fn interactions(
    mut state: ResMut<State<AppState>>,
    mut connection: ResMut<ConnectionRes>,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut windows: ResMut<Windows>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].style.color = colors::WHITE;
                if text.sections[0].value == "Join game" {
                    handle_join_button(connection.as_mut());
                    state.set(AppState::InGame).unwrap();
                }
                if text.sections[0].value == "Settings" {
                    println!("toggling screen mode");
                    toogle_screen_mode(windows.primary_mut());
                }
                if text.sections[0].value == "Exit" {
                    println!("exit");
                }
            }
            Interaction::Hovered => {
                text.sections[0].style.color = colors::GRAY_200;
            }
            Interaction::None => {
                text.sections[0].style.color = colors::WHITE;
            }
        }
    }
}
