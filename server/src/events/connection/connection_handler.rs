use tokio::sync::mpsc;

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
