use bevy::prelude::*;
use rand::Rng;

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("textures/point.png");

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
            (),
        ));
    }
}
