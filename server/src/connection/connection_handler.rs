use tokio::sync::mpsc;
use tokio::sync::oneshot;
use async_trait::async_trait;

#[async_trait]
pub trait ConnectionHandler {
    async fn on_connect(&self, address: String);
    async fn on_disconnect(&self, address: String);
    async fn on_message(&self, address: String, message: String);
}

pub struct ConnectionHandlerImpl {
    pub tx: mpsc::Sender<(String, oneshot::Sender<()>)>,
}

#[async_trait]
impl ConnectionHandler for ConnectionHandlerImpl {
    async fn on_connect(&self, address: String) {
        let (tx, rx) = oneshot::channel();
        self.tx.send((format!("connect:{}", address), tx)).await.unwrap();
        tokio::spawn(async move {
            rx.await.unwrap();
        });
    }

    async fn on_disconnect(&self, address: String) {
        let (tx, rx) = oneshot::channel();
        self.tx.send((format!("disconnect:{}", address), tx)).await.unwrap();
        tokio::spawn(async move {
            rx.await.unwrap();
        });
    }

    async fn on_message(&self, address: String, message: String) {
        let (tx, rx) = oneshot::channel();
        self.tx.send((format!("message:{}:{}", address, message), tx)).await.unwrap();
        tokio::spawn(async move {
            rx.await.unwrap();
        });
    }
}
