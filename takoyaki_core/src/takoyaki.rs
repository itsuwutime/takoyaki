use std::path::PathBuf;
use serde::Deserialize;
use std::fmt::Debug;

use crate::{Error, Cache , build_path , Config, ReadyState, PrintableGrid};


// Type alias
type ReadyFunction = Box<dyn Fn(Cache , Config) -> ReadyState>;
type ExecuteFunction<T> = Box<dyn Fn(T) -> PrintableGrid>;

pub struct Takoyaki<'a , T>
where
    T: for<'de> Deserialize<'de> + Debug
{
    name: &'a str,
    ready: Option<ReadyFunction>,
    execute: Option<ExecuteFunction<T>>,
}

impl<'a , T> Takoyaki<'a , T>
where
    T: for<'de> Deserialize<'de> + Debug
{
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            ready: None,
            execute: None
        }
    }

    pub fn set_ready(&mut self , handler: ReadyFunction) {
        self.ready = Some(handler)
    }

    pub fn set_execute(&mut self , handler: ExecuteFunction<T>) {
        self.execute = Some(handler)
    }

    fn build_path_for_cache(&self) -> Result<PathBuf , std::io::Error> {
        // Get the config path for the root app
        let mut cache = build_path()?;

        // Extend to the cache path
        cache.extend(["cache" , self.name]);

        // Return path
        Ok(cache)
    }

    pub async fn start(&self) -> Result<() , Error> {
        // Prechecks
        if self.ready.is_none() {
            return Err(Error::NoStartFunctionFound)
        }

        if self.execute.is_none() {
            return Err(Error::NoExecuteFunctionFound)
        }

        // Get path
        let cache_path = self.build_path_for_cache().map_err(|_| Error::ConfigDirNotFound)?;

        // Get cache
        let cache = Cache::new(cache_path);
        let config = Config::new(self.name)?;

        // Call the ready function
        let res = self.ready.as_ref().unwrap()(cache , config).resolve::<T>().await?;

        // Graphify the response
        let printable = self.execute.as_ref().unwrap()(res);

        printable.pretty_print();

        Ok(())
    }
}
