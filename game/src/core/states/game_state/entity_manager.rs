use crate::connection::entity_manager::EntityManager;
use bevy::prelude::*;

use super::follower::Folower;

pub fn entity_manager_system(
    entity_manager: Res<EntityManager>,
    mut folowers: Query<&mut Folower>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("textures/point.png");

    let mut my_vec: Vec<String> = [].to_vec();
    folowers.iter_mut().for_each(|t| {
        my_vec.push(t.id.clone());
    });

    for unit in entity_manager.state.units.iter() {
        if !my_vec.contains(&unit.id) {
            println!("created at {:?}, {:?}", unit.position.x, unit.position.y);
            commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_xyz(
                        unit.position.x - 640.0,
                        unit.position.y - 360.0,
                        0.0,
                    ),
                    ..default()
                },
                Folower {
                    id: unit.id.clone(),
                },
            ));
        }
    }
}
