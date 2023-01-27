use std::thread::spawn;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::transform;

use crate::connection;
use crate::entities::unit;
use crate::entities::unit::Position;

use tokio::sync::broadcast;
use tokio::sync::mpsc;

use serde::Deserialize;
use serde::Serialize;
use serde_json::*;

use crate::entities;

#[derive(Resource)]
struct EventReceiver(mpsc::Receiver<String>);

#[derive(Resource)]
struct ClientBroadcast(broadcast::Sender<String>);

#[derive(Deserialize, Debug)]
struct Command {
    username: String,
    action: Action,
}

#[derive(Deserialize, Debug)]
enum Action{
    Join(),
    Spawn(SpawnActionData)
}

#[derive(Deserialize, Debug)]
struct SpawnActionData {
    position: unit::Position,
}

struct Player{
    username: String,
    address: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerReport<'a> {
    username: &'a str,
}

#[derive(Serialize, Debug)]
struct ServerReport<'a> {
    players: Vec<PlayerReport<'a>>,
    units: Vec<unit::UnitReport<'a>>,
}

#[derive(Resource)]
struct PlayerList(Vec<Player>);

pub struct EventProcessorPlugin;

impl Plugin for EventProcessorPlugin {
    fn build(&self, app: &mut App) {
        let (event_receiver, client_broadcast_tx) =
            connection::connection_setup::setup_connection();
        app.insert_resource(EventReceiver(event_receiver))
            .insert_resource(ClientBroadcast(client_broadcast_tx))
            .insert_resource(PlayerList(Vec::new()))
            .add_system_set(
                SystemSet::new()
                    // This prints out "hello world" once every second
                    .with_run_criteria(FixedTimestep::step(1.0 / 30.0))
                    .with_system(client_broadcast),
            )
            .add_system(process_event);
    }
}

fn client_broadcast(
    mut tx: ResMut<ClientBroadcast>,
    units: Query<(&Transform, &unit::Unit)>,
    players: Res<PlayerList>,
) {
    let mut units_vector: Vec<unit::UnitReport> = Vec::new();
    // println!("broadcast system");
    units.iter().for_each(|(transform, unit)| {
        let pos = Position {
            x: transform.translation.x,
            y: transform.translation.y,
        };
        let new_unit_report = unit::UnitReport {
            unit_type: unit.unit_type.as_str(),
            unit_owner: unit.unit_owner.as_str(),
            position: pos,
        };
        units_vector.push(new_unit_report);
    });

    let mut players_vector: Vec<PlayerReport> = Vec::new();
    players.0.iter().for_each(|player| {
        players_vector.push(PlayerReport {
            username: player.username.as_str(),
        })
    });

    let server_report = ServerReport {
        players: players_vector,
        units: units_vector,
    };
    let report_json = serde_json::to_string(&server_report);

    match report_json {
        Ok(report_json) => {
            // println!("sending broadcast {}", report_json);
            match tx.0.send(report_json) {
                Ok(_) => {
                    // println!("sent broadcast");
                }
                Err(error) => {
                    // println!("error sending broadcast {}", error);
                }
            }
        }
        Err(_) => {
            println!("could not serialize broadcast message");
        }
    }
}

fn process_event(
    mut rx: ResMut<EventReceiver>,
    mut commands: Commands,
    mut player_list_resource: ResMut<PlayerList>,
) {
    //let event = task::spawn();
    let event = rx.0.try_recv();
    //let event = task::

    match event {
        Ok(event) => {
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
                    disconnect_player(address, player_list_resource);
                }
                "message" => {
                    let message = parts[2];
                    println!("Received message from {}: {}", address, message);
                    let command: Result<Command> = serde_json::from_str(message);
                    match command {
                        Ok(val) => {
                            println!("{:#?}", val);
                            match val.action{
                                Action::Spawn(data)=> {action_spawn(data, val.username, commands)},
                                Action::Join()=> {action_join(val.username, address.to_string(), player_list_resource)},
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
        Err(_) => {}
    }
}

fn action_spawn(data: SpawnActionData, username: String, mut commands: Commands) {
    println!("Spawning");

    commands.spawn(entities::soldier::new(
        data.position.x,
        data.position.y,
        username,
    ));

    
}

fn action_join(username: String, address: String, mut player_list_resource: ResMut<PlayerList>){
    println!("user {} joined", username);
    player_list_resource.0.push(Player{username: username, address: address});
}

fn disconnect_player(address: &str, mut player_list_resource: ResMut<PlayerList>){
    player_list_resource.0.retain(|player| *player.address != *address);
}
