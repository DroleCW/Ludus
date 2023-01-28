use bevy::prelude::*;

use crate::connection::entity_manager::EntityManager;

use super::follower::Folower;

pub fn move_system(mut folowers: Query<&mut Transform, With<Folower>>, window: Res<Windows>) {
    folowers.iter_mut().for_each(|mut t| {
        let cursor_pos = window
            .get_primary()
            .unwrap()
            .cursor_position()
            .unwrap_or_else(|| Vec2 { x: 0.0, y: 0.0 })
            .clone();
        let mov_dir = Vec3 {
            x: cursor_pos.x - window.get_primary().unwrap().width() / 2.0,
            y: cursor_pos.y - window.get_primary().unwrap().height() / 2.0,
            z: 0.0,
        } - t.translation;
        if mov_dir.length() > 1.0 {
            t.translation += mov_dir.normalize();
        }
    });
}
