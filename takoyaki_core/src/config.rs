use colored::*;
use serde::Deserialize;
use toml::Value;

#[derive(Deserialize)]
pub struct Unicode {
    pub unicode: String,
    pub paint: String
}

#[derive(Deserialize)]
pub struct ConfigType {
    pub unicode: Unicode,
    pub colors: Value
}

#[derive(Default)]
pub struct Config {
    config: Option<ConfigType>
}

impl Config {
    pub fn load(&mut self) -> Result<() , serde_json::Error> {
        let content = std::fs::read_to_string(dirs::config_dir().expect("Cannot get your config directory").join("takoyaki").join("config.toml"));

        if let Err(_) = content {
            println!("{}" , "No config found! Make sure you have ran `takoyaki init`".red());

            return Ok(())
        }

        // It is safe to unwrap `content` from here!
        self.config = Some(toml::from_str(&content.unwrap()).unwrap());

        Ok(())
    }

    pub fn get(&self) -> &ConfigType {
        match &self.config {
            Some(config) => {
                return config
            },
            None => {
                panic!("Must load the config before accessing it! Call `load()` methjod to load the config")
            }
        }
    }
}
