use std::{fs::File, io};
use anyhow::{Context , Result}; 

use crate::{logger::Logger, helpers::get_config_directory};

pub async fn initialize_instance() -> Result<()> {
    let logger = Logger::new();
    let config = get_config_directory()?;

    let config_path = config.join("config.toml");

    logger.success("Initializing a new instance of takoyaki...");
    logger.success(format!("Populating default config for takoyaki at {}" , config_path.display()).as_ref());
    
    // Download file
    let req = reqwest::get("https://raw.githubusercontent.com/kyeboard/takoyaki/main/default.toml").await
        .with_context(|| "Cannot fetch data, make sure you have an active internet connection!")?
        .bytes()
        .await
        .with_context(|| "Error while parsing the context of the response!")?;
    
    // Create confog file
    let mut out = File::create(config_path)
        .with_context(|| "Error while creating a new file!")?
    ;

    // Copy over the contents
    io::copy(&mut req.as_ref(), &mut out).expect("Error while writing to file!");

    logger.success("Successfully populated default config!");
    logger.success("Done!");

    Ok(())
}
