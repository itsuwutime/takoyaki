use std::fs;
use crate::utils::{Logger , get_config_directory};

pub fn unplug(name: &String) {
    // New logger class
    let logger = Logger::new();

    logger.warning("Checking if the plugin is installed...");

    // Build path of the plugin
    let mut plugin_dir = get_config_directory();
    plugin_dir.extend(&["plugins" , name]);

    if !plugin_dir.exists() {
        logger.error("Exiting as the plugin is not installed");

        std::process::exit(0);
    }

    // Remove directory
    fs::remove_dir_all(plugin_dir).expect("Error while deleting the plugin!");

    // Remove cache
    let mut cache_dir = get_config_directory();
    cache_dir.extend(&["cache" , name]);

    fs::remove_dir_all(cache_dir).expect("Error while deleting the plugin!");

    logger.success("Successfully deleted the plugin!");
}
