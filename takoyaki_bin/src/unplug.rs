use std::fs;

use colored::*;

pub fn unplug(name: &String) {
    println!("{} {}" , "==>".green() , "Checking if the plugin is installed...".white());

    let plugin_dir = dirs::config_dir().unwrap().join("takoyaki").join("plugins").join(name);

    if !plugin_dir.exists() {
        return println!("{} {}" , "==>".yellow() , "Exiting as the plugin is not installed".white());
    }

    fs::remove_dir_all(plugin_dir).expect("Error while deleting the plugin!");

    println!("{} {}" , "==>".green() , "Successfully uninstalled the plugin!".white());
}
