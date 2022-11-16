use sha2::{Sha512, Digest};
use uuid::Uuid;

use crate::{logger::Logger, builder::create_new_deployment};

#[derive(Debug)]
pub enum MessageType {
    Auth,
    Undefined,
    Launch
}

#[derive(Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub args: Vec<String>
}

impl Message {
    pub fn new(raw: &str) -> Self {
        let raw = raw.trim_end().trim_start();
        let mut args = raw.split_whitespace().map(String::from).collect::<Vec<String>>();

        match args.remove(0).as_ref() {
            "/auth" => {
                Self {
                    message_type: MessageType::Auth,
                    args
                }
            },
            "/launch" => {
                Self {
                    message_type: MessageType::Launch,
                    args
                }
            },
            &_ => {
                Self {
                    message_type: MessageType::Undefined,
                    args
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
        let passphrase = self.encrypt("512512".to_string());

        if password == passphrase {
            logger.access("Successfully authorized a client.");

            String::from("Successfully authorized!")
        } else {
            logger.fail("Client was delined to get access (invalid_password)");

            String::from("Invalid passphrase! Please recheck the token you have passed")
        }
    }

    pub fn launch(&self , args: &[String]) -> String {
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
