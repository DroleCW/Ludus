use std::thread;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use super::connection_handler;
use tokio::sync::mpsc;
use tokio::sync::broadcast;

pub fn setup_connection() -> (mpsc::Receiver<String>, broadcast::Sender<String>){

    let (tx, mut rx) = mpsc::channel(16);
    let handler = connection_handler::ConnectionHandler { tx };
    let (client_broadcast_tx, _) = broadcast::channel(16);
    let thread_broadcast_tx: broadcast::Sender<String> = client_broadcast_tx.clone();
    let thread_handle = thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(/* async {
                println!("setting up listener");
                let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();
                loop {
                    println!("awaiting connection");
                    let (mut socket, addr) = listener.accept().await.unwrap();
                    let handler = handler.clone();
                    let (socket_rx, mut socket_tx) = socket.into_split();
                    println!("socket setup");
                    handler.on_connect(&addr.to_string()).await;


                    tokio::spawn(async move {   //message receive task
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
                    });

                    let mut new_broadcast_receiver = thread_broadcast_tx.subscribe(); 
                    tokio::spawn(async move {   //message send task
                        loop{
                            match new_broadcast_receiver.recv().await{
                                Ok(message) => {
                                    match socket_tx.write(message.as_bytes()).await{
                                        Ok(bytes) => {if bytes == 0{println!("could not send message to client, closing connection"); break;}}
                                        Err(_) => {println!("error sending message to client, closing connection"); break;}
                                    }
                                },
                                Err(_) => {println!("Error receiveing broadcast message");}
                            }

                        }
                    });
                }
            } */connection_handler::await_connection(thread_broadcast_tx, handler));
    });
    (rx, client_broadcast_tx)
}