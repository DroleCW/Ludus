use bevy::{prelude::*, window::PresentMode};
mod connection;
mod core;

use crate::core::states::{game_state, main_menu_state, state_machine::AppState};
use connection as tcp;
use std::net::TcpStream;
use tcp::connection::ConnectionRes;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Ludus".to_string(),
                    width: 1280.,
                    height: 720.,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                },
                ..default()
            })),
        )
        .add_state(AppState::Menu)
        .insert_resource(ConnectionRes(match TcpStream::connect("localhost:2345") {
            Ok(stream) => {
                println!("created connection");
                Some(stream)
            }
            Err(_) => {
                println!("what a shame");
                None
            }
        }))
        .add_startup_system(setup)
        .add_system(tcp::connection::communicate_system)
        .add_system_set(
            SystemSet::on_enter(AppState::Menu).with_system(main_menu_state::setup_menu),
        )
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(main_menu_state::menu))
        .add_system_set(
            SystemSet::on_exit(AppState::Menu).with_system(main_menu_state::cleanup_menu),
        )
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(game_state::setup_game))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(game_state::move_system))
        .add_system(bevy::window::close_on_esc)
        .run();
}
