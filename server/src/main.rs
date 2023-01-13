use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::thread;

use serde_json::*;
use serde::Deserialize;

mod connection;
use connection::connection_handler;

use bevy::prelude::*;


#[derive(Resource)]
struct EventReceiver(mpsc::Receiver<String>);

#[derive(Deserialize, Debug)]
struct Command {
    username: String,
    action: String,
}


fn main() {
    let (tx, mut rx) = mpsc::channel(8);
    let handler = connection_handler::ConnectionHandler { tx };

    let thread_handle = thread::spawn( move || {
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("setting up listener");
            let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap(); 
            loop{
                println!("awaiting connection");
                let (mut socket, addr) = listener.accept().await.unwrap();
                let handler = handler.clone();
            
                tokio::spawn(async move {
                    let (socket_rx, mut socket_tx) = socket.split();
                    let mut socket_reader = BufReader::new(socket_rx);
                    let mut line = String::new();
                    handler.on_connect(&addr.to_string()).await;
            
                    println!("socket setup");
                    println!("awaiting on socket");
                    while socket_reader.read_line(&mut line).await.unwrap() != 0{
                        println!("received message {}", line);
                        handler.on_message(&addr.to_string(), &line).await;
                        println!("echoing");
                        socket_tx.write(line.as_bytes()).await.unwrap();
                        println!("echoed");
                        line.clear();
                    }
                    println!("disconnected");
                    handler.on_disconnect(&addr.to_string()).await;
                });
            }
        });
    });
    
    
            

    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(EventReceiver(rx))
    .add_startup_system(setup)
    .add_system(process_event)
    .run();
   
    thread_handle.join();
}

fn setup(){
    
}

fn process_event(mut rx: ResMut<EventReceiver>){
    
    //let event = task::spawn();
    let event = rx.0.blocking_recv();
    //let event = task::
    
    match event{
        Some(event) => {
            println!("splitting {}", event);
            let parts: Vec<&str> = event.split("::").collect();
            let event_type = parts[0];
            let address = parts[1];

            match event_type {
                "connect" => {
                    println!("Client connected: {}", address);
                }
                "disconnect" => {
                    println!("Client disconnected: {}", address);
                }
                "message" => {
                    let message = parts[2];
                    println!("Received message from {}: {}", address, message);
                    let command: Result<Command> = serde_json::from_str(message);
                    match command{
                        Ok(val) => {println!("{:#?}", val);},
                        Err(_) => {println!("pooped json");}
                    }
                }
                _ => {
                    println!("Invalid event type: {}", event_type);
                }
            }
        }
        None => {}
    }

}
