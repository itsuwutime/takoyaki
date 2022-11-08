use std::{path::PathBuf, fs::create_dir_all};

pub fn build_path() -> Result<PathBuf , std::io::Error> {
    // Get config dir
    let config_dir = dirs::config_dir();

    // Match 
    match config_dir {
        Some(config) => {
            return Ok(config.join("takoyaki"))
        },
        None => {
            // Get home directory
            let mut home = dirs::home_dir().unwrap();

            home.extend([".config" , "takoyaki"]);

            // Just to make sure not to hit edge cases
            create_dir_all(&home)?;

            return Ok(home)
        }
    }
}

