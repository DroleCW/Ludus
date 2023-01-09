use tokio::net::TcpListener;
use tokio::sync::*;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

mod connection;
use connection::connection_handler;
use crate::connection::connection_handler::ConnectionHandler;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1024);
    let handler = connection_handler::ConnectionHandlerImpl { tx };

    println!("setting up listener");
    let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();

    tokio::spawn(async move{  
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

    while let Some((event, tx)) = rx.recv().await {
        println!("splitting {}", event);
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
                let message = parts[3];
                println!("Received message from {}: {}", address, message);
            }
            _ => {
                println!("Invalid event type: {}", event_type);
            }
        }

        tx.send(()).unwrap();
    }

}
