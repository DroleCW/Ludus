use bevy::prelude::*;

use super::constants::colors;
use crate::connection::send::{send, ConnectionRes};
use crate::core::states::state_machine::AppState;

pub fn handle_join_button(connection: &mut ConnectionRes) {
    println!("join game");
    let msg = b"{\"username\": \"diogodsg\", \"action\": \"join\"}\n";

    match send(connection, msg) {
        Ok(_) => println!("safe"),
        Err(_) => println!("nhe"),
    }
}

pub fn interactions(
    mut state: ResMut<State<AppState>>,
    mut connection: ResMut<ConnectionRes>,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].style.color = colors::WHITE;
                if text.sections[0].value == "Join game" {
                    handle_join_button(connection.as_mut());
                    // state.set(AppState::InGame).unwrap();
                }
                if text.sections[0].value == "Settings" {
                    println!("settings")
                }
                if text.sections[0].value == "Exit" {
                    println!("exit")
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
