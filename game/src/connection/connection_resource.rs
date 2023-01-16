use bevy::prelude::*;

#[derive(Resource)]
pub struct ConnectionRes(pub Option<std::net::TcpStream>);