use std::{path::PathBuf, fs::create_dir_all};
use crate::logger::Logger;

pub fn get_config_directory() -> PathBuf {
    // Get config directory using dirs crate
    let config = dirs::config_dir();
    let logger = Logger::new();

    // Check if the directory is available, or rollback to $HOME/.config (For Android)
    match config {
        Some(config_directory) => {
            return config_directory
        },
        None => {
            // Get home directory
            let home = dirs::home_dir();

            if home.is_none() {
                // Its now hopeless bud
                logger.error("Cannot find your home directory! Make sure you have an $HOME environment variable pointing to your home directory.");

                // Exit process since it is no use to continue
                std::process::exit(0)
            }

            // Build endpoint
            let config_directory = home.unwrap().join(".config");

            // Create directory if not available
            logger.error(create_dir_all(&config_directory).expect_err("Error while creating directory!").to_string().as_ref());

            return config_directory // It is safe to unwrap here!
        }
    }
}
