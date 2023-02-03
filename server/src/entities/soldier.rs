use bevy::prelude::*;

use super::unit;
use uuid::Uuid;

#[derive(Component)]
pub struct Soldier();

pub fn new(x: f32, y: f32, owner: String) -> (Transform, Soldier, unit::Unit) {
    (
        Transform::from_xyz(x, y, 0.0),
        Soldier(),
        unit::Unit {
            id: Uuid::new_v4().to_string(),
            unit_type: "soldier".to_string(),
            unit_owner: owner,
        },
    )
}

fn move_soldier_system(mut soldiers: Query<&mut Transform, With<Soldier>>) {}
