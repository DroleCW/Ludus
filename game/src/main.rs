use bevy::prelude::*;

use rand::Rng;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;

#[derive(Component, Deref)]
struct Velocity(Vec2);

#[derive(Component)]
struct Folower();

#[derive(Resource)]
struct ConnectionRes(Option<std::net::TcpStream>);

fn move_system(mut folowers: Query<&mut Transform, With<Folower>>, window: Res<Windows>) {
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

fn communicate_system(mut connection: ResMut<ConnectionRes>) {
    match &mut connection.0 {
        Some(stream) => {
            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        None => {
            println!("Failed to connect");
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ConnectionRes(match TcpStream::connect("localhost:3333") {
            Ok(stream) => {
                println!("created connection");
                Some(stream)
            }

            Err(_) => {
                println!("what a shame");
                None
            }
        }))
        .add_startup_system(setup)
        .add_system(move_system)
        .add_system(communicate_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}
