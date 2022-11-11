use std::path::PathBuf;
use super::Result;
use crate::hint_cache_path;

pub struct Cache {
    cache_file: PathBuf
}

impl Cache {
    pub fn new(app_name: &str) -> Result<Self> {
        Ok(Self {
            cache_file: hint_cache_path(app_name)?
        })
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self {
            cache_file: path
        }
    }
}

