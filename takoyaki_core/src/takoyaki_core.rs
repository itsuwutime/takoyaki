// Import dependencies
use serde::{Deserialize, Serialize};

// Import in built modules
use crate::{Plugin, Config, Cache};

// Main tokayaki entrypoint
#[derive(Default)]
pub struct Takoyaki<'a , T , U>
where 
    // T represents the type of response it handles
    T: for<'de> Deserialize<'de> + Default + Serialize,
    // U represents the type of config it handles
    U: for <'de> Deserialize<'de>
{
    plugin: Option<&'a dyn Plugin<'a , T , U>> // The plugin!
}

// Add functions
impl<'a , T , U> Takoyaki<'a , T , U>
where 
    T: for<'de> Deserialize<'de> + Default + Serialize,
    U: for <'de> Deserialize<'de>
{
    // Plugs in the plugin
    pub fn plug(&mut self , plugin: &'a dyn Plugin<'a , T , U>) {
        self.plugin = Some(plugin)
    }

    // Main function
    pub async fn start(&self) -> Result<() , reqwest::Error> {
        // Check if the pluugin is plugged
        if self.plugin.is_none() {
            // PANIC!
            panic!("Must set a plugin before executing! Call the `plug()` method to add in a plugin")
        }

        // Get the config for the application
        let mut config = Config::default();

        // Load the config
        config.load().expect("Unable to parse the config. Make sure you dont have any unnecessary characters");

        // Make a reference of the plugin
        let plugin = self.plugin.as_ref().expect("Cannot create a reference of the plugin");

        // Get the name of the plugin (will be used to identify the config and cache endpoint)
        let name = plugin.name();

        // Create new instance of cache
        let cache = Cache::new(name.to_string());

        // Get the data that will be rendered as graph
        let data = plugin.ready(
            Config::parse_from_name(name).unwrap(), 
            cache.clone()
        ).resolve::<T>(cache).await?; 
        
        // Get the `PrintableGraph` struct and pretty print it
        plugin.execute(data).pretty_print(config).unwrap();

        // OK!
        Ok(())
    }
}

