use std::path::PathBuf;

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
}

