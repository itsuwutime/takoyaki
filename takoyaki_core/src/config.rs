// Import dependencies
use colored::*;
use serde::Deserialize;
use toml::Value;
use crate::get_config_directory;
use anyhow::Result;

// Unicode config
#[derive(Deserialize)]
pub struct Unicode {
    pub unicode: String,
    pub paint: String
}

// Main config type
#[derive(Deserialize)]
pub struct ConfigType {
    pub unicode: Unicode,
    pub colors: Value
}

// main config struct
#[derive(Default)]
pub struct Config {
    config: Option<ConfigType>
}

// Functions
impl Config {
    // Loads in app config (main)
    pub fn load(&mut self) -> Result<()> {
        // Read the content
        let content = std::fs::read_to_string(
            get_config_directory()?
                .join("config.toml")
        );

        // Check if it is not a bomb
        if content.is_err() {
            panic!("{}" , "No config found! Make sure you have ran `takoyaki init`".red())
        }

        // It is safe to unwrap `content` from here!
        self.config = Some(toml::from_str(&content?)?);

        // OK!
        Ok(())
    }

    // Get config for a specific plugin
    pub fn parse_from_name<T>(name: &str) -> Result<T> 
    where
        T: for <'de> Deserialize<'de>
    {
        // Get config dir
        let mut config = get_config_directory()?;

        // Beautifulllll
        config.extend(&["plugins" , name , "config.toml"]);

        // Get the file content
        let content = std::fs::read_to_string(
            config
        );

        // Check if it is not a bomb
        if content.is_err() {
            panic!("{}" , "No config found! Make sure you have the plugin installed".red());
        }

        // It is safe to unwrap `content` from here!
        Ok(toml::from_str(&content?)?)
    }

    pub fn get(&self) -> &ConfigType {
        match &self.config {
            Some(config) => {
                config
            },
            None => {
                panic!("Must load the config before accessing it! Call `load()` method to load the config")
            }
        }
    }
}
