use std::{io::{Cursor, Write}, fs::Permissions, os::unix::prelude::PermissionsExt, collections::HashMap};

use inquire::Text;
use reqwest::StatusCode;
use colored::*;
use serde::Deserialize;
use throbber::Throbber;

#[derive(Deserialize)]
pub struct PlugConfig {
    pub name: String,
    pub binary: String,
    pub requires: Vec<String>
}

pub async fn plug(repository: &String , branch: &String , path: &String) {
    // Notify users that takoyaki is fetching the metadata
    println!("{} {}" , "==>".green() , "Fetching metadata for the plugin...".white());

    // Build the end path
    let path = format!("https://raw.githubusercontent.com/{}/{}{}/takoyaki.plugin.toml" , repository , branch, path);

    // Make request
    let content = reqwest::get(path).await.unwrap();

    // Check if the response succeeded
    if content.status() != StatusCode::OK {
        // Notify users that they have either provide wrong args or the plugin is bugged
        return println!("{} {}" , "==>".red() , "Cannot find the plugin at the specific path! Make sure the path does not start and end with `/` or check if the repo exists".white());
    }

    // Notify users that metadata has been successfully retrived
    println!("{} {}" , "==>".green() , "Parsing metadata...".white());

    // Parse 
    let parsed = toml::from_str::<PlugConfig>(content.text().await.unwrap().as_ref());

    // Check if the parsing was success
    if parsed.is_err() {
        // Erroring out means that there were some issues with the toml file
        println!("{} {}" , "==>".red() , "Invalid configuration! Make sure that it is a valid toml. You must report this issue to the developer of the plugin!".white());
    }

    // Notify users about downloading the plugin
    // println!("{} {}" , "==>".green() , "Downloading plugin...".white());
    let mut throbber = Throbber::new()
        .message("Downloading plugin...".to_string());
    
    throbber.start();

    let plugin_meta = parsed.unwrap();

    // Build path for plugin directory
    let plugin_dir = dirs::config_dir().unwrap().join("takoyaki").join("plugins").join(&plugin_meta.name);

    // Download plugin
    let plugin = reqwest::get(plugin_meta.binary.clone()).await.unwrap();

    // Create directory for that plugin
    std::fs::create_dir_all(&plugin_dir).expect("Error while creating a directory!");

    // Create a new file for the executable
    let mut file = std::fs::File::create(plugin_dir.join("start")).unwrap();

    // Set executable permissions
    file.set_permissions(Permissions::from_mode(0o711)).expect("Error while setting up executable permissions for the binary");

    // Create a new cursor
    let mut cursor = Cursor::new(plugin.bytes().await.unwrap());

    // Download the file
    std::io::copy(&mut cursor , &mut file).expect("Error while writing to file");

    throbber.success("Downloaded the plugin".to_string());

    throbber.end();

    // Take input of the config
    println!("{} {}" , "==>".green() , "Setting up config for the plugin...".white());

    let mut config: HashMap<String , String> = HashMap::new();

    // Iterate through all the required configs
    for option in plugin_meta.requires.iter() {
        let value = Text::new(format!("Enter {}:" , option).as_ref()).prompt().unwrap();

        config.insert(option.to_owned() , value);
    }

    let mut file = std::fs::File::create(plugin_dir.join("config.toml")).unwrap();
    
    file.write_all(toml::to_string(&config).unwrap().as_bytes()).expect("Error while writing to cache!");

    // Create cache folder
    std::fs::create_dir_all(dirs::config_dir().unwrap().join("takoyaki").join("cache").join(&plugin_meta.name)).expect("Unable to create directory");

    println!("{} {}" , "==>".green() , format!("Successfully installed the plugin. You can now use it by running `takoyaki use {}`" , plugin_meta.name).white());
}

