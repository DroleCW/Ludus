use super::send;
use bevy::prelude::*;
use std::net::TcpStream;
pub struct ConnectionPlugin;
use serde::Serialize;
#[derive(Serialize, Debug)]
pub struct ConnectionEvent<T> {
    pub username: String,
    pub action: String,
    pub data: T,
}

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(send::ConnectionRes(
            match TcpStream::connect("localhost:2345") {
                Ok(stream) => {
                    println!("created connection");
                    Some(stream)
                }
                Err(_) => {
                    println!("what a shame");
                    None
                }
            },
        ));
    }
}
