use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Folower();
pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("../textures/point.png");

    for _ in 0..10 {
        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(0.0..200.0),
                    rand::thread_rng().gen_range(0.0..200.0),
                    0.0,
                ),
                ..default()
            },
            Folower(),
        ));
    }
}

pub fn move_system(mut folowers: Query<&mut Transform, With<Folower>>, window: Res<Windows>) {
    folowers.iter_mut().for_each(|mut t| {
        let cursor_pos = window
            .get_primary()
            .unwrap()
            .cursor_position()
            .unwrap_or_else(|| Vec2 { x: 0.0, y: 0.0 })
            .clone();
        //println!("{}", cursor_pos);
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
