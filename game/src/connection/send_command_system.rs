use bevy::prelude::*;
use std::io::{Read, Write};
use std::str::from_utf8;
use super::connection_resource::ConnectionRes;

pub fn send_command_system(mut connection: ResMut<ConnectionRes>) {
    match &mut connection.0 {
        Some(stream) => {
            println!("reading");
            let msg = b"{\"username\": \"diogodsg\", \"action\": \"join\"}\n";
            println!("Sending message");
            stream.write(msg).unwrap();
        }
        None => {
            println!("Failed to connect");
        }
    }
}
