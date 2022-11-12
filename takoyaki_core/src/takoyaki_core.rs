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

    pub async fn start(&self) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn start_without_start_function() {
        let takoyaki = Takoyaki::<serde_json::Value>::new();

        assert_eq!(takoyaki.start().await.unwrap_err() , Error::StartFunctionNotSet);
    }

    #[tokio::test]
    pub async fn start_without_execute_function() {
        let mut takoyaki = Takoyaki::<serde_json::Value>::new();

        takoyaki.set_ready(Box::new(|| {
            ReadyState::new()
        }));

        assert_eq!(takoyaki.start().await.unwrap_err() , Error::ExecuteFunctionNotSet);
    }

    #[tokio::test]
    pub async fn state_is_unset() {
        let mut takoyaki = Takoyaki::<serde_json::Value>::new();

        takoyaki.set_ready(Box::new(|| {
            ReadyState::new()
        }));

        takoyaki.set_execute(Box::new(|_| {
            PrintableGrid::new()
        }));

        assert_eq!(takoyaki.start().await.unwrap_err() , Error::StateIsUnset);
    }

    #[tokio::test]
    pub async fn state_from_reqwest() {
        let mut takoyaki = Takoyaki::<serde_json::Value>::new();

        takoyaki.set_ready(Box::new(|| {
            ReadyState::from_reqwest(
                reqwest::Client::new()
                    .get("https://jsonplaceholder.typicode.com/todos/1")
            )
        }));

        takoyaki.set_execute(Box::new(|data| {
            assert_eq!(data.as_object().unwrap().get("id").unwrap().as_u64().unwrap() , 1);

            PrintableGrid::new()
        }));

        assert!(takoyaki.start().await.is_ok());
    }
}

