use bevy::prelude::*;
use std::io::{self, Read, Write};
use std::str::from_utf8;

#[derive(Resource)]
pub struct ConnectionRes(pub Option<std::net::TcpStream>);

pub fn send(connection: &mut ConnectionRes, msg: &[u8]) -> Result<String, io::Error> {
    match &mut connection.0 {
        Some(stream) => {
            let mut data = [0; 1024];
            stream.write(msg).unwrap();
            match stream.read(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("Reply: {}", text);
                    Ok(text.to_string())
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    Err(e)
                }
            }
        }
        None => {
            println!("Failed to connect");
            Err(io::Error::new(io::ErrorKind::NotFound, "Failed to connect"))
        }
    }
}
