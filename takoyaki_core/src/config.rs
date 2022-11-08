use std::path::PathBuf;
use serde::Deserialize;

use crate::{build_path, errors::Errors};

pub struct Config {
    path: PathBuf
}

impl Config {
    pub fn new(name: &str) -> Result<Self , Errors> {
        let mut takoyaki_path = build_path().map_err(|_| Errors::ConfigDirNotFound)?;

        takoyaki_path.extend(["plugins" , name , "config.toml"]);

        Ok(Self {
            path: takoyaki_path
        })
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn get<T>(&self) -> Result<T , Errors>
    where 
        T: for<'de> Deserialize<'de>
    {
        // Check if the config exists
        if !self.exists() {
            return Err(Errors::NoConfigFound)
        }

        // Read the config as a raw string
        let raw = std::fs::read_to_string(&self.path).map_err(|_| Errors::ReadError)?;

        // Convert to the `T` type
        toml::from_str(raw.as_ref()).map_err(|_| Errors::BuggedConfig)
    }
}

