use std::path::PathBuf;
use serde::Deserialize;

use crate::Error;

#[derive(Debug)]
pub struct Cache {
    cache_dir: PathBuf
}

impl<'a> Cache {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            cache_dir
        }
    }

    pub fn exists(&'a self) -> bool {
        self.cache_dir.exists()
    }

    pub fn create(&'a self) -> Result<() , std::io::Error> {
        std::fs::create_dir_all(&self.cache_dir)
    } 

    pub fn retrieve<T>(&self) -> Result<T , Error> 
    where
        T: for<'de> Deserialize<'de>
    {
        let raw = std::fs::read_to_string(&self.cache_dir).map_err(|_| Error::ReadError)?;

        serde_json::from_str(&raw).map_err(Error::SerializeJSONError)
    }
}

