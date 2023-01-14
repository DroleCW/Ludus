use bevy::prelude::*;

use crate::core::states::state_machine::AppState;

use super::constants::colors;

pub fn interactions(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].style.color = colors::WHITE;
                state.set(AppState::InGame).unwrap();
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
