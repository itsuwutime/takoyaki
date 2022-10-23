use std::{path::PathBuf, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Cache {
    cache_endpoint: PathBuf
}

impl Cache {
    pub fn new(plug_name: String) -> Self {
        Self {
            cache_endpoint: dirs::config_dir().unwrap().join("takoyaki").join("cache").join(plug_name).join("cache.json")
        }
    }

    pub fn exists(&self) -> bool {
        self.cache_endpoint.exists()
    }

    pub fn get<T>(&self) -> Result<T , serde_json::Error>
    where
        T: for<'de> Deserialize<'de>
    {
        if !self.exists() {
            panic!("Cache does not exist! Most probably you need to populate the cache first!")
        }

        serde_json::from_str(std::fs::read_to_string(self.cache_endpoint.clone()).unwrap().as_ref())
    }

    pub fn populate<T>(&self , cache: T) -> Result<() , std::io::Error>
    where
        T: Serialize
    {
        let mut file = std::fs::File::create(&self.cache_endpoint)?;

        file.write_all(serde_json::to_string(&cache).unwrap().as_bytes())?;

        Ok(())
    }
}
