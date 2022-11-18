use colored::*;
use std::fs::create_dir_all;

use crate::LOGGER;

pub fn setup_workspace() {
    LOGGER.success("Setting up directories");

    // Get the root directory
    let takoyaki_root = dirs::home_dir().unwrap().join(".takoyaki");

    // Create the directory just in case if it does not exist
    create_dir_all(&takoyaki_root).unwrap();

    // Setup dirs
    let build_cache_dir = takoyaki_root.clone().join("build");

    // Create the directory 
    create_dir_all(&build_cache_dir).unwrap();

    // Success message showing the path that will be used
    LOGGER.render(format!("Creating builds at: {}" , build_cache_dir.display()).magenta().bold());

    // Setup deployment dirs
    let deployments_dir = takoyaki_root.clone().join("deployments");

    // Create the directory 
    create_dir_all(&deployments_dir).unwrap();

    LOGGER.render(format!("Creating deployment logs at: {}" , deployments_dir.display()).magenta().bold());

    // Setup deployment dirs
    let built_plugins_dir = takoyaki_root.clone().join("plugins");

    // Create the directory 
    create_dir_all(&built_plugins_dir).unwrap();

    // Success message showing the path that will be used
    LOGGER.render(format!("Saving built plugins at: {}" , built_plugins_dir.display()).magenta().bold());
}
