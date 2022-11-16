use std::net::SocketAddr;
use sha2::{Sha512, Digest};
use uuid::Uuid;
use crate::{Logger, create_new_deployment, STATE};

#[derive(Debug)]
pub enum MessageType {
    Auth,
    Undefined,
    Launch
}

#[derive(Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub args: Vec<String>,
    pub incoming_addr: SocketAddr
}

impl Message {
    pub fn new(raw: &str, incoming_addr: SocketAddr) -> Self {
        let raw = raw.trim_end().trim_start();
        let mut args = raw.split_whitespace().map(String::from).collect::<Vec<String>>();

        match args.remove(0).as_ref() {
            "/auth" => {
                Self {
                    message_type: MessageType::Auth,
                    args,
                    incoming_addr,
                }
            },
            "/launch" => {
                Self {
                    message_type: MessageType::Launch,
                    args,
                    incoming_addr,
                }
            },
            &_ => {
                Self {
                    message_type: MessageType::Undefined,
                    args,
                    incoming_addr,
                }
            }
        }
    }

    pub fn encrypt(&self , raw: String) -> String {
        let mut encrypter = Sha512::new();

        encrypter.update(raw);

        base64::encode(encrypter.finalize())
    }

    pub fn authenticate(&self , args: &[String]) -> String {
        let logger = Logger::new();

        let password = self.encrypt(format!("512{}512" , args.iter().next().unwrap()));
        let passphrase = self.encrypt("512somerandompinfordev512".to_string());

        if password == passphrase {
            logger.access("Successfully authorized a client.");

            STATE.lock().allow_ip(self.incoming_addr);

            String::from("Successfully authorized!")
        } else {
            logger.fail("Client was delined to get access (invalid_password)");

            String::from("Invalid passphrase! Please recheck the token you have passed")
        }
    }

    pub fn launch(&self , args: &[String]) -> String {
        let logger = Logger::new();

        if !STATE.lock().is_allowed(self.incoming_addr) {
            logger.fail("Plugin deployment was rejected (ip_not_authorized)");

            return String::from("Plugin deployment was rejected (ip_not_authorized)")
        }
        let logger = Logger::new();
        let uuid = Uuid::new_v4().to_string();

        tokio::spawn(create_new_deployment(uuid.clone() , args[0].clone() , args[1].clone() , args[2].clone()));

        logger.success(&format!("Creating a new build with uuid of {}" , uuid));

        uuid
    }

    pub fn undefined(&self) -> String {
        String::from("Invalid command!")
    }

    pub fn respond(&self) -> String {
        match self.message_type {
            MessageType::Auth => {
                self.authenticate(&self.args)
            },
            MessageType::Launch => {
                self.launch(&self.args)
            },
            MessageType::Undefined => {
                self.undefined()
            }
        }
    }
}
