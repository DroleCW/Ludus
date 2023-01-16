use bevy::prelude::*;

use super::connection;

use tokio::sync::mpsc;

use serde::Deserialize;
use serde_json::*;

#[derive(Resource)]
struct EventReceiver(mpsc::Receiver<String>);

#[derive(Deserialize, Debug)]
struct Command {
    username: String,
    action: String,
}


pub struct EventProcessorPlugin;

impl Plugin for EventProcessorPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(EventReceiver(connection::connection_setup::setup_connection()))
        .add_system(process_event);
    }
}


fn process_event(mut rx: ResMut<EventReceiver>) {
    //let event = task::spawn();
    let event = rx.0.blocking_recv();
    //let event = task::

    match event {
        Some(event) => {
            println!("splitting {}", event);
            let parts: Vec<&str> = event.split("::").collect();
            let event_type = parts[0];
            let address = parts[1];

            match event_type {
                "connect" => {
                    println!("Client connected: {}", address);
                }
                "disconnect" => {
                    println!("Client disconnected: {}", address);
                }
                "message" => {
                    let message = parts[2];
                    println!("Received message from {}: {}", address, message);
                    let command: Result<Command> = serde_json::from_str(message);
                    match command {
                        Ok(val) => {
                            println!("{:#?}", val);
                        }
                        Err(_) => {
                            println!("pooped json");
                        }
                    }
                }
                _ => {
                    println!("Invalid event type: {}", event_type);
                }
            }
        }
        None => {}
    }
}