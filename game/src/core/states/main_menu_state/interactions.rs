use bevy::prelude::*;

use super::constants::colors;

pub fn interactions(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].style.color = colors::WHITE;
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
