use anyhow::Result;

use crate::helpers::get_config_directory;

pub fn refresh_plugins() -> Result<()> {
    // Remove all the cache first
    let config_directory = get_config_directory()?;

    // Iterate through the installed plugins and get the name
    for name in std::fs::read_dir(config_directory.join("plugins"))? {
        let file_name = name?.file_name();

        // Remove the cache first
        std::fs::remove_dir_all(config_directory.join("cache").join(&file_name))?;

        // Use the plugin to fill up the new cache
        crate::use_plugin::use_plugin(&file_name.to_str().unwrap().to_string())?;
    }

    println!("DONE!");

    Ok(())
}
