use sha2::{Sha256 , Digest};

use crate::LOGGER;

pub struct Message<'a> {
    pub command: &'a str,
    pub args: Vec<&'a str>
}

impl<'a> Message<'a> {
    pub fn from_raw(raw: &'a str) -> Self {
        let mut args: Vec<&str> = raw.split_whitespace().collect();
        let command = args.remove(0);

        Self {
            command,
            args
        }
    }

    pub fn respond(&self) -> &str {
        match self.command {
            "/auth" => {
                self.authenticate()
            },
            _ => {
                self.undefined()
            }
        }
    }

    fn encrypt_with_salt(&self , raw: &str , salt: &str) -> String {
        // Create a new sha256 gen
        let mut encrypter = Sha256::new();

        // Update with the value
        encrypter.update(format!("{}{}{}" , salt , raw , salt));

        // Return the final result
        base64::encode(encrypter.finalize())
    }

    pub fn authenticate(&self) -> &str {
        // Check if the user has provided atleast one argument
        if self.args.len() < 1 {
            return "Must provide atleast one argument for the token"
        }

        // Get the encrypted version of the master key
        let master_key = self.encrypt_with_salt(
            option_env!("MASTER_KEY").unwrap_or(""),
            "512"
        );

        // Get the encrypted version of the passed token
        let passphrase = self.encrypt_with_salt(self.args[0] , "512");

        if master_key == passphrase {
            LOGGER.warning("A client has been successfully authorized");

            "Successfully authorized"
        } else {
            LOGGER.warning("A client authentication request was rejected (invalid_passphrase)");

            "Invalid passphrase"
        }
    }

    pub fn launch(&self) -> &str {
        if self.args.len() < 3 {
            return "Must provide atleast three arguments. To know more about this command, visit - https://takoyaki.kyeboard.me/docs/websockets/commands#launch"
        }



        ""
    }

    pub fn undefined(&self) -> &str {
        "Invalid command! You can see a list of supported commands here - https://takoyaki.kyeboard.me/docs/websockets/commands"
    }
}

