use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use futures_channel::mpsc::unbounded;
use futures_util::{StreamExt, TryStreamExt , future, pin_mut};
use tokio::net::{TcpListener, TcpStream};

use crate::{Error, Result, LOGGER, Message};

pub struct Server {
    port: u16
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self {
            port
        }
    } 

    pub fn get_addr(&self) -> SocketAddr {
        SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127 , 0 , 0 , 1)),
            self.port
        )
    }

    pub async fn handle_client(&self , stream: TcpStream , incoming_addr: SocketAddr) -> Result<()> {
        // Show a warning saying that a new client has made a connection request
        LOGGER.warning(&format!("Incoming connection request from {}..." , &incoming_addr));

        // Accept the connection
        let ws_stream = tokio_tungstenite::accept_async(stream).await.map_err(|_| Error::HandshakeError)?;

        // Create a new peer
        let (tx , rx) = unbounded();
        let (outgoing , incoming) = ws_stream.split();

        // Listen for incoming messages
        let broadcast_incoming = incoming.try_for_each(|msg| {
            let message_raw = msg.into_text().map_err(|_| Error::CannotParseRecievedData).unwrap();

            let message = Message::from_raw(&message_raw);

            tx.unbounded_send(tungstenite::Message::Text(message.respond().to_string())).unwrap();

            future::ok(())
        });

        // Recieve from others
        let recieve = rx.map(Ok).forward(outgoing);

        // Pin
        pin_mut!(broadcast_incoming , recieve);

        // Wait for them to exit
        future::select(broadcast_incoming , recieve).await;

        // Print an disconnect message 
        LOGGER.warning(&format!("Client with the address of {} has disconnected" , incoming_addr));

        Ok(())
    }

    pub async fn listen(&'static self) -> Result<()> {
        // Get address
        let addr = self.get_addr();

        // Success message to show where the server is running
        LOGGER.success(&format!("Starting server on address {}..." , &addr));

        // Create a TCP listener on the specified port
        let stream = TcpListener::bind(addr).await.map_err(|_| Error::CannotListenOnAddress)?;

        // Successfully started!
        LOGGER.success(&format!("Successfully started server on address {}..." , &addr));

        while let Ok((stream , addr)) = stream.accept().await {
            tokio::spawn(self.handle_client(stream , addr));
        }

        Ok(())
    }
}

