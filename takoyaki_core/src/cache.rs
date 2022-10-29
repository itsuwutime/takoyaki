// Import dependencies
use serde::{Deserialize, Serialize};
use std::{io::Write, path::PathBuf};
use anyhow::Result;

use crate::get_config_directory;

// Cache struct
#[derive(Clone)]
pub struct Cache {
    cache_endpoint: PathBuf,
}

impl Cache {
    pub fn new(plug_name: String) -> Self {
        // Get config directory
        let mut cache = get_config_directory().unwrap();

        // Extend till the cache endpoint 
        cache.extend(&["cache" , plug_name.as_ref() , "cache.json"]);

        Self {
            cache_endpoint: cache
        }
    }

    pub fn exists(&self) -> bool {
        self.cache_endpoint.exists()
    }

    pub fn get<T>(&self) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Check if the cache file exists
        if !self.exists() {
            panic!("Cache does not exist! Probably you need to populate the cache first!")
        }

        // Convert it to T type
        Ok(serde_json::from_str(
            std::fs::read_to_string(self.cache_endpoint.clone())?
                .as_ref(),
        )?)
    }

    pub fn populate<T>(&self, cache: T) -> Result<()>
    where
        T: Serialize,
    {
        // Create cache folder to prevent error
        std::fs::create_dir_all(&self.cache_endpoint.parent().unwrap())?;

        // Create a new file
        let mut file = std::fs::File::create(&self.cache_endpoint)?;

        // Write the data
        file.write_all(serde_json::to_string(&cache)?.as_bytes())?;

        // Ok!
        Ok(())
    }
}
