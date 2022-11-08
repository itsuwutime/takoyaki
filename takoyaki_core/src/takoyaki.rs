use std::path::PathBuf;

use crate::{Errors, Cache , build_path , Config, ReadyState};

pub struct Takoyaki<'a> {
    name: &'a str,
    ready: Option<Box<dyn Fn(Cache , Config) -> ReadyState >>,
    execute: Option<Box<dyn Fn()>>,
}

impl<'a> Takoyaki<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            ready: None,
            execute: None
        }
    }

    pub fn set_ready(&mut self , handler: Box<dyn Fn(Cache , Config) -> ReadyState>) {
        self.ready = Some(handler)
    }

    pub fn set_execute(&mut self , handler: Box<dyn Fn()>) {
        self.execute = Some(handler)
    }

    fn build_path_for_cache(&self) -> Result<PathBuf , std::io::Error> {
        // Get the config path for the root app
        let mut cache = build_path()?;

        // Extend to the cache path
        cache.extend(["cache" , self.name]);

        // Return path
        return Ok(cache)
    }

    pub async fn start(&self) -> Result<() , Errors> {
        // Prechecks
        if self.ready.is_none() {
            return Err(Errors::NoStartFunctionFound)
        }

        if self.execute.is_none() {
            return Err(Errors::NoExecuteFunctionFound)
        }

        // Get path
        let cache_path = self.build_path_for_cache().map_err(|_| Errors::ConfigDirNotFound)?;

        // Get cache
        let cache = Cache::new(cache_path);
        let config = Config::new(self.name)?;

        // Call the ready function
        let res = self.ready.as_ref().unwrap()(cache , config).resolve().await?;

        Ok(())
    }
}
