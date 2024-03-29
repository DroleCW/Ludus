use bevy::prelude::*;

pub mod entity_manager;
pub mod follower;
pub mod input_system;
pub mod move_system;
pub mod setup;

use super::state_machine::AppState;
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup::setup_game))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(move_system::move_system)
                    .with_system(input_system::mouse_button_input)
                    .with_system(entity_manager::entity_manager_system),
            )
            .add_system(bevy::window::close_on_esc);
    }
}
