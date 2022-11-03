use std::fs;
use crate::utils::{Logger , get_config_directory};

pub fn unplug(name: &String) {
    // New logger class
    let logger = Logger::new();
    let mut config_dir = get_config_directory();

    logger.warning("Checking if the plugin is installed...");

    // Build path of the plugin
    let plugin_dir = &mut config_dir;
    plugin_dir.extend(&["plugins" , name]);

    if !plugin_dir.exists() {
        logger.error("Exiting as the plugin is not installed");

        std::process::exit(0);
    }

    // Remove directory
    fs::remove_dir_all(plugin_dir).expect("Error while deleting the plugin!");

    // Remove cache
    let cache_dir = &mut config_dir;
    cache_dir.extend(&["cache" , name]);

    fs::remove_dir_all(cache_dir).expect("Error while deleting the plugin!");

    logger.success("Successfully deleted the plugin!");
}
