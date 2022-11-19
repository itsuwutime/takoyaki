use std::fs::create_dir_all;
use colored::Colorize;
use crate::LOGGER;

pub fn setup() {
    LOGGER.success("Setting up your directories...");

    // Get root
    let takoyaki_root = dirs::home_dir().unwrap().join(".takoyaki");

    // Get the build directory
    let build = takoyaki_root.clone().join("build");

    // Create the directory
    create_dir_all(&build).unwrap();

    // Print the build directory
    LOGGER.render(format!("Creating build cache at {}" , build.display()).magenta().bold());

    // Get the deployments directory
    let deployments = takoyaki_root.clone().join("deployments");

    // Create the directory
    create_dir_all(&deployments).unwrap();

    // Print the build directory
    LOGGER.render(format!("Creating deployments logs at {}" , deployments.display()).magenta().bold());

    // Get the deployments directory
    let plugins = takoyaki_root.clone().join("plugins");

    // Create the directory
    create_dir_all(&plugins).unwrap();

    // Print the build directory
    LOGGER.render(format!("Creating built plugins at {}" , plugins.display()).magenta().bold());

}
