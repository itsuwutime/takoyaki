use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};

use crate::{message::Message, logger::Logger};

pub struct Server {

}

impl Server {
    pub fn new() -> Self {
        Self {

        }
    }

    pub async fn handle_client(&'static self , stream: TcpStream) {
        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Error during the websocket handshake")
        ;

        let (tx , rx) = unbounded();
        let (outgoing , incoming) = ws_stream.split();

        let broadcast_incoming = incoming.try_for_each(|msg| {
            let message = msg.into_text().unwrap();
            let message = Message::new(&message);

            tx.unbounded_send(tungstenite::Message::Text(message.respond())).unwrap();

            future::ok(())
        });

        let receive_from_others = rx.map(Ok).forward(outgoing);
        pin_mut!(broadcast_incoming, receive_from_others);

        future::select(broadcast_incoming, receive_from_others).await;
    }

    pub async fn start(&'static self , _allowed_list: Vec<&str>) {
        let logger = Logger::new();
        let socket = TcpListener::bind("127.0.0.1:8000").await.expect("Failed to bind");

        logger.success("Server is up and running. Waiting for incoming connections...");

        while let Ok((stream , addr)) = socket.accept().await {
            logger.info(&format!("Incoming request from address {}" , addr));

            tokio::spawn(self.handle_client(stream));
        }
    }
}

