use super::communicate_system;
use bevy::prelude::*;
use std::net::TcpStream;

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(communicate_system::communicate_system)
            .insert_resource(communicate_system::ConnectionRes(match TcpStream::connect(
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
