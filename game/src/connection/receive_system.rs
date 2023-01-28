use super::connection_resource::ConnectionRes;
use super::entity_manager::EntityManager;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json::*;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Unit {
    pub unit_type: String,
    pub unit_owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitReport {
    pub unit_type: String,
    pub unit_owner: String,
    pub position: Position,
}

pub struct Player {
    username: String,
    address: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerReport {
    pub username: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerReport {
    pub players: Vec<PlayerReport>,
    pub units: Vec<UnitReport>,
}
pub fn receive_command_system(
    mut connection: ResMut<ConnectionRes>,
    mut entity_manager: ResMut<EntityManager>,
) {
    match &mut connection.0 {
        Some(stream) => {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let string_buf = String::from_utf8(buffer.to_vec()).unwrap();
            let parts: Vec<&str> = string_buf.split_terminator('\0').collect();

            let result: Result<ServerReport> = serde_json::from_str(parts[0]);
            match result {
                Ok(rep) => {
                    entity_manager.state = rep;
                }
                Err(err) => println!("{:?}", err),
            }
        }
        None => {
            println!("Failed to connect");
        }
    }
}
