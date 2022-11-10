use std::{path::PathBuf, io::Write};
use serde::Deserialize;

use crate::{Error , hint_cache_path};

#[derive(Debug)]
pub struct Cache {
    cache_path: PathBuf
}

impl<'a> Cache {
    pub fn from_name(plug_name: &str) -> Self {
        Self {
            cache_path: hint_cache_path(plug_name)
        }
    }

    pub fn from_path(cache_path: PathBuf) -> Self {
        Self {
            cache_path
        }
    }

    pub fn exists(&'a self) -> bool {
        self.cache_path.exists()
    }

    pub fn create(&'a self) -> Result<() , std::io::Error> {
        std::fs::create_dir_all(&self.cache_path)
    } 

    pub fn retrieve<T>(&self) -> Result<T , Error> 
    where
        T: for<'de> Deserialize<'de>
    {
        let raw = std::fs::read_to_string(&self.cache_path).map_err(|_| Error::ReadError)?;

        serde_json::from_str(&raw).map_err(Error::SerializeJSONError)
    }

    pub fn populate_as_str(&self , raw: &str) -> Result<() , Error> {
        let mut file = std::fs::File::create(&self.cache_path).map_err(Error::CacheNotCreated)?;

        file.write_all(raw.as_bytes()).map_err(Error::WriteError)?;

        Ok(())
    }
}

