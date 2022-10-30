use std::fs;
use crate::logger::Logger;
use crate::helpers::get_config_directory;
use anyhow::Result;

pub fn unplug(name: &String) -> Result<()> {
    // New logger class
    let logger = Logger::new();
    
    logger.success("Checking if the plugin is installed...");

    // Build path of the plugin
    let mut plugin_dir = get_config_directory()?;
    plugin_dir.extend(&["plugins" , name]);

    if !plugin_dir.exists() {
        logger.warning("Exiting as the plugin is not installed");

        std::process::exit(0);
    }

    // Remove directory
    fs::remove_dir_all(plugin_dir).expect("Error while deleting the plugin!");

    // Remove cache
    let mut cache_dir = get_config_directory()?;
    cache_dir.extend(&["cache" , name]);

    fs::remove_dir_all(cache_dir).expect("Error while deleting the plugin!");

    logger.success("Successfully deleted the plugin!");
    
    Ok(())
}
