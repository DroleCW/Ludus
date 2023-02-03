use bevy::prelude::Component;
use serde::Deserialize;
use serde::Serialize;
use serde_json::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Unit {
    pub id: String,
    pub unit_type: String,
    pub unit_owner: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitReport<'a> {
    pub id: &'a str,
    pub unit_type: &'a str,
    pub unit_owner: &'a str,
    pub position: Position,
}
