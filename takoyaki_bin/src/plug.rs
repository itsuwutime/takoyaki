use std::{io::{Cursor, Write}, fs::Permissions, os::unix::prelude::PermissionsExt, collections::HashMap};

use inquire::Text;
use reqwest::StatusCode;
use serde::Deserialize;
use crate::logger::Logger;
use throbber::Throbber;
use anyhow::Context;
use reqwest::Result;

#[derive(Deserialize , Debug)]
pub struct PlugConfig {
    pub name: String,
    pub binary: String,
    pub requires: Vec<String>
}

pub async fn plug(name: String) -> Result<()> {
    let logger = Logger::new();

    logger.success("Fetching metadata for the plugin...");

    // Get metadata
    let metadata = crate::metadata::get_metadata(name.as_ref()).await;

    // Check if there is no reqwest error
    if metadata.is_err() {
        logger.error("You don't have an active internet connection! Try connecting to a network.");

        std::process::exit(0);
    }

    // // Unwrap the metadata
    let metadata = metadata.unwrap();

    // Check if metadata is not None
    if metadata.is_none() {
        logger.error(
            format!(
                "No plugin named {} found! If you think this is an error, please report it here - https://www.github.com/kyeboard/takoyaki/issues/new",
                name
            ).as_ref()
        );

        std::process::exit(0)
    }

    // Final unwrap
    let metadata = metadata.unwrap();

    // Make request
    let content = reqwest::get(&metadata.config_url).await?;
    
    // Check if the response succeeded
    if content.status() != StatusCode::OK {
        // Notify users that they have either provide wrong args or the plugin is bugged
        logger.error("Invalid config url found! Make sure you have an active internet connection or report this issue");

        std::process::exit(0)
    }
    
    logger.success("Parsing metadata...");

    println!("{:?}" , metadata.config_url.clone());
    
    // Parse 
    let parsed = toml::from_str::<PlugConfig>(content.text().await?.as_ref()).unwrap(); // Since the config will be reviewed by me, there is relatively low chance of plugin's config bugged.
    
    // Notify users about downloading the plugin
    logger.success("Downloading the plugin...");
    
    println!("{:?}" , parsed);

    // let mut throbber = Throbber::new()
    //     .message("Downloading plugin...".to_string());
    
    // throbber.start();
    
    // let plugin_meta = parsed.unwrap();
    
    // // Build path for plugin directory
    // let plugin_dir = dirs::config_dir().unwrap().join("takoyaki").join("plugins").join(&plugin_meta.name);
    
    // // Download plugin
    // let plugin = reqwest::get(plugin_meta.binary.clone()).await.unwrap();
    
    // // Create directory for that plugin
    // std::fs::create_dir_all(&plugin_dir).expect("Error while creating a directory!");
    
    // // Create a new file for the executable
    // let mut file = std::fs::File::create(plugin_dir.join("start")).unwrap();
    
    // // Set executable permissions
    // file.set_permissions(Permissions::from_mode(0o711)).expect("Error while setting up executable permissions for the binary");
    
    // // Create a new cursor
    // let mut cursor = Cursor::new(plugin.bytes().await.unwrap());
    //
    // // Download the file
    // std::io::copy(&mut cursor , &mut file).expect("Error while writing to file");
    //
    // throbber.success("Downloaded the plugin".to_string());
    //
    // throbber.end();
    //
    // // Take input of the config
    // println!("{} {}" , "==>".green() , "Setting up config for the plugin...".white());
    //
    // let mut config: HashMap<String , String> = HashMap::new();
    //
    // // Iterate through all the required configs
    // for option in plugin_meta.requires.iter() {
    //     let value = Text::new(format!("Enter {}:" , option).as_ref()).prompt().unwrap();
    //
    //     config.insert(option.to_owned() , value);
    // }
    //
    // let mut file = std::fs::File::create(plugin_dir.join("config.toml")).unwrap();
    //
    // file.write_all(toml::to_string(&config).unwrap().as_bytes()).expect("Error while writing to cache!");
    //
    // // Create cache folder
    // std::fs::create_dir_all(dirs::config_dir().unwrap().join("takoyaki").join("cache").join(&plugin_meta.name)).expect("Unable to create directory");
    //
    // println!("{} {}" , "==>".green() , format!("Successfully installed the plugin. You can now use it by running `takoyaki use {}`" , plugin_meta.name).white());

    Ok(())
}

