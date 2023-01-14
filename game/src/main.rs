mod connection;
mod core;
mod settings;

use crate::core::states::state_machine::StatePlugin;
use bevy::prelude::*;
use connection::connection::ConnectionPlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(settings::window::window_settings())))
        .add_plugin(ConnectionPlugin)
        .add_plugin(StatePlugin)
        .add_startup_system(setup)
        .run();
}
