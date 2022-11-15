use sha2::{Sha512, Digest};
use uuid::Uuid;

#[derive(Debug)]
pub enum MessageType {
    Auth,
    Undefined,
    Launch
}

#[derive(Debug)]
pub struct Message<'a> {
    pub message_type: MessageType,
    pub args: Vec<&'a str>
}

impl<'a> Message<'a> {
    pub fn new(raw: &'a str) -> Self {
        let raw = raw.trim_end().trim_start();
        let mut args = raw.split_whitespace().collect::<Vec<&str>>();

        match args.remove(0) {
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

    pub fn authenticate(&self , args: &[&'a str]) -> String {
        let password = self.encrypt(format!("512{}512" , args.iter().next().unwrap()));
        let passphrase = self.encrypt("512512".to_string());

        if password == passphrase {
            String::from("Successfully authorized!")
        } else {
            String::from("Invalid passphrase! Please recheck the token you have passed")
        }
    }

    pub fn launch(&self) -> String {
        Uuid::new_v4().to_string()
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
                self.launch()
            },
            MessageType::Undefined => {
                self.undefined()
            }
        }
    }
}
