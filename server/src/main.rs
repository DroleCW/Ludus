use tokio::net::TcpListener;
use tokio::sync::*;

mod connection;
use connection::connection_handler;
use crate::connection::connection_handler::ConnectionHandler;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1024);
    let handler = connection_handler::ConnectionHandlerImpl { tx };

    tokio::spawn(async move {
        while let Some((event, tx)) = rx.recv().await {
            let parts: Vec<&str> = event.split(":").collect();
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
                }
                _ => {
                    println!("Invalid event type: {}", event_type);
                }
            }

            tx.send(()).unwrap();
        }
    });

    handler.on_connect("127.0.0.1:8080".to_string());
    handler.on_message("127.0.0.1:8080".to_string(), "Hello".to_string());
    handler.on_disconnect("127.0.0.1:8080".to_string());
}
