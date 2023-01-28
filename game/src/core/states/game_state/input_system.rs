use crate::connection::connection_resource::ConnectionRes;
use crate::connection::send::send;
use bevy::prelude::*;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Serialize, Debug)]
pub struct SpawnActionData {
    position: Position,
}

#[derive(Serialize, Debug)]
pub enum Action {
    Join(),
    Spawn(SpawnActionData),
}

#[derive(Serialize, Debug)]
pub struct ConnectionEvent {
    pub username: String,
    pub action: Action,
}

pub fn mouse_button_input(buttons: Res<Input<MouseButton>>, mut connection: ResMut<ConnectionRes>) {
    if buttons.just_released(MouseButton::Left) {
        let action_data = SpawnActionData {
            position: Position { x: 2.0, y: 2.0 },
        };
        let spawn_action = ConnectionEvent {
            username: "diogodsg".to_string(),
            action: Action::Spawn(action_data),
        };

        let mut stringified_action = serde_json::to_string(&spawn_action).unwrap();
        stringified_action.push('\n');

        println!("string is: {}", stringified_action);
        send(connection.as_mut(), stringified_action.as_bytes());
    }
}
