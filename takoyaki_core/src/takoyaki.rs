use std::path::PathBuf;
use serde::Deserialize;
use std::fmt::Debug;

use crate::{Error, Cache , Config, ReadyState, PrintableGrid};

// Type alias
type ReadyFunction = Box<dyn for<'a> Fn(&'a Cache , &'a Config) -> ReadyState<'a>>;
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

    pub async fn start(&self) -> Result<() , Error> {
        // Prechecks
        if self.ready.is_none() {
            return Err(Error::NoStartFunctionFound)
        }

        if self.execute.is_none() {
            return Err(Error::NoExecuteFunctionFound)
        }

        // Get cache
        let cache = Cache::from_name(self.name);
        let config = Config::new(self.name)?;

        // Call the ready function
        let res = self.ready.as_ref().unwrap()(&cache , &config).resolve::<T>(&cache).await?;

        // Graphify the response
        let printable = self.execute.as_ref().unwrap()(res);

        printable.pretty_print()?;

        Ok(())
    }
}

