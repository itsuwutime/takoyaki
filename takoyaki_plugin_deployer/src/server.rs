use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};

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
            future::ok(())
        });

        let receive_from_others = rx.map(Ok).forward(outgoing);
        pin_mut!(broadcast_incoming, receive_from_others);

        future::select(broadcast_incoming, receive_from_others).await;
    }

    pub async fn start(&'static self , allowed_list: Vec<&str>) {
        let socket = TcpListener::bind("127.0.0.1:8000").await.expect("Failed to bind");

        while let Ok((stream , addr)) = socket.accept().await {
            tokio::spawn(self.handle_client(stream));
        }
    }
}

