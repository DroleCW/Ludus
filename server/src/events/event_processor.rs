use bevy::prelude::*;

use crate::connection;

use tokio::sync::mpsc;
use tokio::sync::broadcast;

use serde::Deserialize;
use serde_json::*;

use crate::entities;

#[derive(Resource)]
struct EventReceiver(mpsc::Receiver<String>);

#[derive(Resource)]
struct ClientBroadcast(broadcast::Sender<String>);

#[derive(Deserialize, Debug)]
struct Command {
    username: String,
    action: String,
    data: String
}

#[derive(Deserialize, Debug)]
struct SpawnActionData{
    x: f32,
    y: f32
}


pub struct EventProcessorPlugin;

impl Plugin for EventProcessorPlugin{
    fn build(&self, app: &mut App){
        let (event_receiver, client_broadcast_tx) = connection::connection_setup::setup_connection();
        app.insert_resource(EventReceiver(event_receiver))
        .insert_resource(ClientBroadcast(client_broadcast_tx))
        .add_system(process_event);
    }
}


fn process_event(mut rx: ResMut<EventReceiver>, mut commands: Commands) {
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
                            match val.action.as_str(){
                                "spawn"=> {action_spawn(val.data, commands)},
                                "join"=> {action_join(val.username)},
                                _ => {}
                            }
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

fn action_spawn(data: String, mut commands: Commands){
    let spawn_data: Result<SpawnActionData> = serde_json::from_str(data.as_str());
    match spawn_data {
        Ok(position) => {commands.spawn(entities::soldier::new(position.x, position.y));},
        Err(_) => {println!("wrong spawn action data");}
        
    }
}

fn action_join(username: String){
    println!("user {} joined", username)
}