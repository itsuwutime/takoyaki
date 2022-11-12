use serde::Deserialize;

use crate::{ReadyState, PrintableGrid, Result, Error};

pub struct Takoyaki<'a , T> 
where
    T: for<'de> Deserialize<'de>
{
    ready: Option<Box<dyn Fn() -> ReadyState>>,
    execute: Option<Box<dyn Fn(T) -> PrintableGrid<'a>>>
}

impl<'a , T> Takoyaki<'a , T> 
where
    T: for<'de> Deserialize<'de>
{
    pub fn new() -> Self {
        Self {
            ready: None,
            execute: None
        }
    }

    pub fn set_ready(&mut self , handler: Box<dyn Fn() -> ReadyState>) {
        self.ready = Some(handler);
    }

    pub fn set_execute(&mut self , handler: Box<dyn Fn(T) -> PrintableGrid<'a>>) {
        self.execute = Some(handler)
    }

    pub async fn execute(&self) -> Result<()> {
        // Get all the handlers
        let start = self.ready.as_ref().ok_or(Error::StartFunctionNotSet)?;
        let execute = self.execute.as_ref().ok_or(Error::ExecuteFunctionNotSet)?;

        // Get the state of the plugin
        let state = start();

        // Resolve the state to get the data
        let data = state.resolve::<T>().await?;

        // Send it to execute function to get a printable grid
        let printable = execute(data);

        // Pretty print the table
        printable.pretty_print(None)?;

        // Ok!
        Ok(())
    }
}
