use crate::connection::connection::ConnectionEvent;
use crate::connection::send::{send, ConnectionRes};
use bevy::prelude::*;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Position {
    x: f32,
    y: f32,
}
#[derive(Serialize, Debug)]

struct SpawnSoldierEvent {
    position: Position,
}

pub fn mouse_button_input(buttons: Res<Input<MouseButton>>, mut connection: ResMut<ConnectionRes>) {
    if buttons.just_released(MouseButton::Left) {
        let action_data = SpawnSoldierEvent {
            position: Position { x: 2.0, y: 2.0 },
        };
        let spawn_action = ConnectionEvent {
            username: "diogodsg".to_string(),
            action: "spawn".to_string(),
            data: "awfa".to_string(),
            // data: serde_json::to_string(&action_data).unwrap(),
        };

        // let msg = b"{\"username\": \"diogodsg\", \"action\": \"spawn\", \"data\": \"aa\"}\n";
        let stringified_action = serde_json::to_string(&spawn_action);
        // let a = b"{\"username\":\"diogodsg\",\"action\":\"spawn\",\"data\":{\"position\":{\"x\":2.0,\"y\":2.0}}}";
        let msg =
            b"{\"username\": \"diogodsg\", \"action\": \"spawn\", \"data\": \"{\\\"position\\\":{\\\"x\\\":2.0,\\\"y\\\":2.0}}\"}\n";

        send(connection.as_mut(), msg);

        // match stringified_action {
        //     Ok(stream) => {
        //         println!("{}", stream);
        //         send(connection.as_mut(), msg);
        //     }
        //     Err(_) => println!("failed to send spawn troop event"),
        // }
    }
}
