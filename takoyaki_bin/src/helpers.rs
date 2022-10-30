use std::{path::PathBuf, fs::create_dir_all};
use crate::logger::Logger;
use anyhow::{Context , Result};

pub fn get_config_directory() -> Result<PathBuf> {
    // Get config directory using dirs crate
    let config = dirs::config_dir();
    let logger = Logger::new();

    // Check if the directory is available, or rollback to $HOME/.config (For Android)
    let endpoint = match config {
        Some(config_directory) => {
            config_directory.join("takoyaki")
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
            let config_directory = home.unwrap().join(".config");  // It is safe to unwrap here!

            // Create directory if not available
            create_dir_all(&config_directory)
                .with_context(|| "Error while creating a new directory!")?
            ;
            
            config_directory.join("takoyaki")
        }
    };

    create_dir_all(&endpoint)
        .with_context(|| "Error while creating a new directory!")?
    ;

    Ok(endpoint)
}
