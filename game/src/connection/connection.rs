use super::connection_resource::ConnectionRes;
use super::entity_manager::EntityManager;

use super::receive_system;
use super::receive_system::ServerReport;

use bevy::prelude::*;
use std::net::TcpStream;
pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityManager {
            state: ServerReport {
                players: vec![],
                units: vec![],
            },
        })
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
        .add_system(receive_system::receive_command_system);
    }
}
