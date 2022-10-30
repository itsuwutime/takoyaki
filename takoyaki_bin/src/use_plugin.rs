use std::process::Command;
use anyhow::Result;
use crate::helpers::get_config_directory;
use crate::logger::Logger;

pub fn use_plugin(name: &String) -> Result<()> {
    let mut executable = get_config_directory()?;
    let logger = Logger::new();

    executable.extend(&["plugins" , name.as_ref() , "start"]);

    if !executable.exists() {
        logger.error("The plugin does not exist!");

        std::process::exit(0)
    }

    Command::new("sh")
        .arg("-c")
        .arg(executable)
        .spawn()
        .expect("Error while running the plugin!");

    Ok(())
}

