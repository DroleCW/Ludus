use bevy::prelude::*;
use std::io::{Read, Write};
use std::str::from_utf8;

#[derive(Resource)]
pub struct ConnectionRes(pub Option<std::net::TcpStream>);

pub fn communicate_system(mut connection: ResMut<ConnectionRes>) {
    match &mut connection.0 {
        Some(stream) => {
            println!("reading");
            let mut data = [0; 64];
            let msg = b"{\"username\": \"diogodsg\", \"action\": \"join\"}\n";
            println!("Sending Hello");
            stream.write(msg).unwrap();
            println!("Sent olaaa, awaiting reply...");

            match stream.read(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Reply: {}", text);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        None => {
            println!("Failed to connect");
        }
    }
}
