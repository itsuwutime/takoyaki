use serde::Deserialize;
use crate::{plugin::Plugin, config::Config};

pub struct Takoyaki<'a , T , U>
where 
    T: for<'de> Deserialize<'de> + Default,
    U: for <'de> Deserialize<'de>
{
    plugin: Option<&'a dyn Plugin<'a , T , U>>
}

impl<'a , T , U> Takoyaki<'a , T , U>
where 
    T: for<'de> Deserialize<'de> + Default,
    U: for <'de> Deserialize<'de>
{
    pub fn new() -> Self {
        Self {
            plugin: None
        }
    }

    pub fn plug(&mut self , plugin: &'a dyn Plugin<'a , T , U>) {
        self.plugin = Some(plugin)
    }

    pub async fn start(&self) -> Result<() , reqwest::Error> {
        if self.plugin.is_none() {
            panic!("Must set a plugin before executing! Call the `plug()` method to add in a plugin")
        }

        let mut config = Config::default();

        config.load().expect("Unable to parse the config. Make sure you dont have any unnecessary characters");

        let plugin = self.plugin.as_ref().expect("Cannot create a reference of the plugin");

        let name = plugin.name();

        let data = plugin.ready(Config::parse_from_name(name).unwrap()).resolve::<T>().await?; // Get ready
        
        Ok(plugin.execute(data).pretty_print(config))
    }
}

