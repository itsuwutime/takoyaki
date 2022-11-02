use std::process::{Command, Stdio};
use crate::utils::{get_config_directory , Logger};

pub fn run(name: &String , no_output: bool) {
    // Get the variables ready
    let mut executable = get_config_directory();
    let logger = Logger::new();

    executable.extend(&["plugins" , name.as_ref() , "start"]);

    if !executable.exists() {
        logger.error("The plugin does not exist!");

        std::process::exit(0)
    }

    let mut command = Command::new("sh");

    command
        .arg("-c")
        .arg(executable)
    ;

    if no_output {
        command
            .stderr(Stdio::null())
            .stdout(Stdio::null());
    }

    command
        .spawn()
        .unwrap()
        .wait()
        .unwrap_or_else(|_| {
            logger.error("Error while executing the plugin");

            std::process::exit(1)
        })

    ;
}

