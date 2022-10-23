// Import dependencies
use colored::*;
use serde::Deserialize;
use toml::Value;

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
    pub fn load(&mut self) -> Result<() , serde_json::Error> {
        // Read the content
        let content = std::fs::read_to_string(
            dirs::config_dir()
                .expect("Cannot get your config directory")
                .join("takoyaki")
                .join("config.toml")
            );

        // Check if it is not a bomb
        if content.is_err() {
            panic!("{}" , "No config found! Make sure you have ran `takoyaki init`".red())
        }

        // It is safe to unwrap `content` from here!
        self.config = Some(toml::from_str(&content.unwrap()).unwrap());

        // OK!
        Ok(())
    }

    // Get config for a specific plugin
    pub fn parse_from_name<T>(name: &str) -> Result<T , serde_json::Error> 
    where
        T: for <'de> Deserialize<'de>
    {
        // Get the file content
        let content = std::fs::read_to_string(
            dirs::config_dir()
                .expect("Cannot get your config directory")
                .join("takoyaki")
                .join("plugins")
                .join(name)
                .join("config.toml")
            );

        // Check if it is not a bomb
        if content.is_err() {
            panic!("{}" , "No config found! Make sure you have the plugin installed".red());
        }

        // It is safe to unwrap `content` from here!
        Ok(toml::from_str(&content.unwrap()).unwrap())
    }

    pub fn get(&self) -> &ConfigType {
        match &self.config {
            Some(config) => {
                config
            },
            None => {
                panic!("Must load the config before accessing it! Call `load()` methjod to load the config")
            }
        }
    }
}
