use super::game_state::GameStatePlugin;
use super::main_menu_state::MainMenuStatePlugin;
use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    InGame,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu)
            .add_plugin(MainMenuStatePlugin)
            .add_plugin(GameStatePlugin);
    }
}
