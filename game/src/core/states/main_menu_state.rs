use bevy::prelude::*;

#[path = "../ui/transparent-button.rs"]
mod button;
#[path = "../../settings/constants.rs"]
mod constants;
#[path = "./main_menu_state/interactions.rs"]
mod interactions;
#[path = "./main_menu_state/layout.rs"]
mod layout;
#[path = "./main_menu_state/setup.rs"]
mod setup;

use super::state_machine::AppState;
pub struct MainMenuStatePlugin;

impl Plugin for MainMenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup::setup_menu))
            .add_system_set(
                SystemSet::on_update(AppState::Menu).with_system(interactions::interactions),
            )
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(setup::cleanup_menu));
    }
}
