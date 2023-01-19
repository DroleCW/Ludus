use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct ConnectionHandler {
    pub tx: mpsc::Sender<String>
}

impl ConnectionHandler {
    pub async fn on_connect(&self, address: &String) {
        self.tx.send(format!("connect::{}", address)).await.unwrap();
    }

    pub async fn on_disconnect(&self, address: &String) {
        self.tx.send(format!("disconnect::{}", address)).await.unwrap();
    }

    pub async fn on_message(&self, address: &String, message: &String) {
        self.tx.send(format!("message::{}::{}", address, message)).await.unwrap();
    }
}

pub async fn await_connection(client_broadcast_tx: broadcast::Sender<String>, handler: ConnectionHandler){
    println!("setting up listener");
    let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();
    loop {
        println!("awaiting connection");
        let (mut socket, addr) = listener.accept().await.unwrap();
        let handler = handler.clone();
        let (socket_rx, mut socket_tx) = socket.into_split();
        println!("socket setup");
        handler.on_connect(&addr.to_string()).await;


        tokio::spawn(await_message(socket_rx, addr, handler));

        let aaa = client_broadcast_tx.clone();
        tokio::spawn(send_client_broadcast(aaa, socket_tx));
    }
}

async fn await_message(socket_rx: OwnedReadHalf, addr: SocketAddr, handler: ConnectionHandler){
    let mut socket_reader = BufReader::new(socket_rx);
    let mut line = String::new();
    println!("awaiting on socket");
    loop{
        match socket_reader.read_line(&mut line).await{
            Ok(bytes) => {
                if bytes != 0{
                    println!("received message {}", line);
                    handler.on_message(&addr.to_string(), &line).await;
                    line.clear();
                }
                else{
                    print!("nothing received, closing connection");
                    break;
                }
            }
            Err(error) => {
                println!("error receiving, closing connection: {}", error); 
                break;
            }
        }
    }
    println!("disconnected");
    handler.on_disconnect(&addr.to_string()).await;
}

async fn send_client_broadcast(broadcast_sender: broadcast::Sender<String>, mut socket_tx: OwnedWriteHalf){   //message send task
    let mut broadcast_receiver = broadcast_sender.subscribe();
    loop{
        match broadcast_receiver.recv().await{
            Ok(message) => {
                match socket_tx.write(message.as_bytes()).await{
                    Ok(bytes) => {if bytes == 0{println!("could not send message to client, closing connection"); break;}}
                    Err(_) => {println!("error sending message to client, closing connection"); break;}
                }
            },
            Err(_) => {println!("Error receiveing broadcast message");}
        }

    }
}