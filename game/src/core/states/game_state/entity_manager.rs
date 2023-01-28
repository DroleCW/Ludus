use crate::connection::entity_manager::EntityManager;
use bevy::prelude::*;

pub fn entity_manager_system(entity_manager: Res<EntityManager>) {
    println!("UNITS REPORT: \n");
    for unit in entity_manager.state.units.iter() {
        println!(
            "{} owned by {} in position ({}, {})",
            unit.unit_type, unit.unit_owner, unit.position.x, unit.position.y
        );
    }
    println!("PLAYERS REPORT: \n");
    for player in entity_manager.state.players.iter() {
        println!("{}", player.username);
    }
}
