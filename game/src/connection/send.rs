use bevy::prelude::*;
use std::io::Write;

#[derive(Resource)]
pub struct ConnectionRes(pub Option<std::net::TcpStream>);

pub fn send(connection: &mut ConnectionRes, msg: &[u8]) {
    match &mut connection.0 {
        Some(stream) => {
            println!("{:?}", msg);
            let a = stream.write(msg).unwrap();
            println!("{}", a);
        }
        None => {
            println!("Failed to connect");
        }
    }
}
