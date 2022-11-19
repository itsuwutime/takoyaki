use anyhow::Result;
use colored::*;
use futures_channel::mpsc::unbounded;
use futures_util::{StreamExt, TryStreamExt, pin_mut};
use tokio::net::{TcpListener, TcpStream};
use std::net::{SocketAddr , IpAddr , Ipv4Addr};

use crate::{LOGGER, CommandManager, Authenticate , Command, Deploy};

pub struct Server<'a> {
    commands: CommandManager<'a>
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        let mut commands = CommandManager::new();

        // Add commands
        commands.register_command(Box::new(Authenticate::new()));
        commands.register_command(Box::new(Deploy::new()));

        Self {
            commands
        }
    }

    pub async fn handle_client(&'static self , stream: TcpStream , addr: SocketAddr) -> Result<()> {
        // A log message showing the connection
        LOGGER.render(format!("Incoming connection request from address {}" , addr).magenta());

        // Accept incoming connection
        let ws_client = tokio_tungstenite::accept_async(stream).await?;

        // Create a new peer
        let (tx , rx) = unbounded();
        let (outgoing , incoming) = ws_client.split();

        // Listen for incoming messages
        let broadcase_incoming = incoming.try_for_each(|msg| async {
            let message = self.commands.parse_from_raw(&msg.into_text().unwrap()).await;

            tx.unbounded_send(tungstenite::Message::Text(message.to_string())).unwrap();

            Ok(())
        });

        // Recieve from peers
        let recieve_from_others = rx.map(Ok).forward(outgoing);

        // Pin
        pin_mut!(broadcase_incoming , recieve_from_others);

        // Wait for either one of them to exit
        futures_util::future::select(broadcase_incoming, recieve_from_others).await;

        // Show a message that the peer has disconnected
        LOGGER.success(&format!("Client with the address of {} has been disconnected" , addr));

        Ok(())
    }

    pub fn leak(self) -> &'static mut Self {
        Box::leak(Box::new(self))
    }

    pub async fn listen(&'static self , port: u16) -> Result<()> {
        // Get the address to listen to
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127 , 0 , 0 , 1)),
            port
        );

        // A little success message
        LOGGER.success(&format!("Starting server on address {}" , addr));

        // Create a TCP listener
        let stream = TcpListener::bind(addr).await?;

        // UwU
        LOGGER.success(&format!("Successfully started the server on address {}" , addr));

        // Listen for incoming requests
        while let Ok((stream , addr)) = stream.accept().await {
            tokio::spawn(self.handle_client(stream , addr));
        }

        // Ok!
        Ok(())
    }
}

