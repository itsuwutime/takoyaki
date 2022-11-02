use std::{io::{Cursor, Write}, fs::Permissions, os::unix::prelude::PermissionsExt, collections::HashMap};

use inquire::Text;
use reqwest::StatusCode;
use serde::Deserialize;
use crate::utils::{Logger , get_config_directory};

#[derive(Deserialize , Debug)]
pub struct PlugConfigData {
    pub name: String,
    pub bin_url: String,
    pub requires: Vec<String>
}

#[derive(Deserialize , Debug)]
pub struct PlugConfig {
    pub description: PlugConfigData
}

fn no_internet_connection(logger: &Logger) -> ! {
    logger.error("You don't have an active internet connection! Try connecting to a network.");

    std::process::exit(0)
}

fn plugin_not_found(name: String , logger: &Logger) -> ! {
    logger.error(
        format!(
            "No plugin named {} found! If you think this is an error, please report it here - https://www.github.com/kyeboard/takoyaki/issues/new",
            name
        ).as_ref()
    );

    std::process::exit(0)
}

pub async fn plug(name: String) {
    let logger = Logger::new();

    logger.success("Fetching metadata for the plugin...");

    // Get metadata
    let metadata = crate::utils::get_metadata(name.as_ref()).await
        .unwrap_or_else(|_| { no_internet_connection(&logger) })
        .unwrap_or_else(|| { plugin_not_found(name, &logger) })
    ;

    // Make request
    let content = reqwest::get(&metadata.config_url).await.unwrap_or_else(|_| no_internet_connection(&logger));

    // Check if the response succeeded
    if content.status() != StatusCode::OK {
        // Notify users that they have either provide wrong args or the plugin is bugged
        no_internet_connection(&logger)
    }

    logger.success("Parsing metadata...");

    // Parse 
    let parsed = toml::from_str::<PlugConfig>(content.text().await.unwrap().as_ref()).unwrap(); // Since the config will be reviewed by me, there is relatively low chance of plugin's config bugged.

    // Notify users about downloading the plugin
    logger.success("Downloading the plugin... (it may take a while)");

    // Build path for plugin directory
    let mut plugin_dir = get_config_directory();

    // Extend till the endpoint
    plugin_dir.extend(&["plugins" , parsed.description.name.as_ref()]);

    // Download plugin
    let plugin = reqwest::get(parsed.description.bin_url).await.unwrap_or_else(|_| {
        logger.error(
            "Error while making the request! Either you don't have an active internet connection or the plugin is bugged. You should report this error"
        );

        std::process::exit(0);
    });

    // Create directory for that plugin
    std::fs::create_dir_all(&plugin_dir).unwrap_or_else(|_| { 
        logger.error(format!("Error while creating a directory at {}" , plugin_dir.display()).as_ref());

        std::process::exit(1)
    });

    // Create a new file for the executable
    let mut file = std::fs::File::create(plugin_dir.join("start")).unwrap();

    // Set executable permissions
    file.set_permissions(Permissions::from_mode(0o711)).unwrap_or_else(|_| {
        logger.error("Error while setting up executable permissions for the binary!");

        std::process::exit(0)
    });

    // Create a new cursor
    let mut cursor = Cursor::new(plugin.bytes().await.unwrap());

    // Download the file
    std::io::copy(&mut cursor , &mut file).unwrap_or_else(|_| {
        logger.error("Error while downloading the binary!");

        std::process::exit(1)
    });

    logger.success("Successfully downloaded the binary!");
    logger.success("Setting up config for the plugin...");

    let mut config: HashMap<String , String> = HashMap::new();

    // Iterate through all the required configs
    for option in parsed.description.requires.iter() {
        let value = Text::new(format!("Enter {}:" , option).as_ref()).prompt().unwrap_or_else(|_| {
            logger.error("Exiting...");

            std::process::exit(0);
        });

        config.insert(option.to_string() , value);
    }

    let mut file = std::fs::File::create(plugin_dir.join("config.toml")).unwrap_or_else(|_| {
        logger.error("Error while creating a new file!");

        std::process::exit(1)
    });


    file.write_all(
        toml::to_string(&config)
            .unwrap()
            .as_bytes()
    )
    .unwrap_or_else(|_| {
        logger.error("Error while writing to file!")
    });

    logger.success(format!("Successfully installed the plugin! You can now use it by running `takoyaki use {}`" , parsed.description.name).as_ref());
}

