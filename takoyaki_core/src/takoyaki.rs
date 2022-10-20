use serde::Deserialize;
use crate::plugin::Plugin;

pub struct Takoyaki<'a , T>
where 
    T: for<'de> Deserialize<'de> + Default
{
    plugin: Option<&'a dyn Plugin<'a , T>>
}

impl<'a , T> Takoyaki<'a , T>
where 
    T: for<'de> Deserialize<'de> + Default
{
    pub fn new() -> Self {
        Self {
            plugin: None
        }
    }

    pub fn plug(&mut self , plugin: &'a dyn Plugin<'a , T>) {
        self.plugin = Some(plugin)
    }

    pub async fn start(&self) -> Result<() , reqwest::Error> {
        if self.plugin.is_none() {
            panic!("Must set a plugin before executing! Call the `plug()` method to add in a plugin")
        }

        let plugin = self.plugin.as_ref().expect("Cannot create a clone of the plugin!");

        let data = plugin.ready().resolve::<T>().await?; // Get ready
        
        Ok(plugin.execute(data).pretty_print())
    }
}

