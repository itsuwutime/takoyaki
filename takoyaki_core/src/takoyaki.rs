use serde::Deserialize;

use crate::plugin::Plugin;

pub struct Takoyaki<'a , T>
where 
    T: for<'de> Deserialize<'de> + Default
{
    plugin: Option<Box<dyn Plugin<'a , T>>>
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

    pub fn plug(&mut self , plugin: Box<dyn Plugin<'a , T>>) {
        self.plugin = Some(plugin)
    }

    pub async fn start(&self) {
        if self.plugin.is_none() {
            panic!("Must set a plugin before executing! Call the `plug` method to add in a plugin")
        }

        let data = self.plugin.as_ref().unwrap().as_ref().ready().resolve::<T>().await.unwrap(); // Get ready
        
        self.plugin.as_ref().unwrap().as_ref().execute(data)
    }
}

