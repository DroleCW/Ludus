use super::{send_command_system, connection_resource};
use bevy::prelude::*;
use std::net::TcpStream;

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(send_command_system::send_command_system)
            .insert_resource(connection_resource::ConnectionRes(match TcpStream::connect(
                "localhost:2345",
            ) {
                Ok(stream) => {
                    println!("created connection");
                    Some(stream)
                }
                Err(_) => {
                    println!("what a shame");
                    None
                }
            }));
    }
}
