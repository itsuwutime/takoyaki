use std::{path::PathBuf, fs::create_dir_all, process::{Command, Stdio}};
use crate::utils::Logger;

pub fn get_config_directory() -> PathBuf {
    // Get config directory using dirs crate
    let config = dirs::config_dir();
    let logger = Logger::new();

    // Check if the directory is available, or rollback to $HOME/.config (For Android)
    let endpoint = match config {
        Some(config_directory) => {
            config_directory.join("takoyaki")
        },
        None => {
            // Get home directory
            let home = dirs::home_dir().unwrap_or_else(|| {
                // Its now hopeless bud
                logger.error("Cannot find your home directory! Make sure you have an $HOME environment variable pointing to your home directory.");

                // Exit process since it is no use to continue
                std::process::exit(0)
            });

            // Build endpoint
            let config_directory = home.join(".config");  // It is safe to unwrap here!

            // Create directory if not available
            create_dir_all(&config_directory).unwrap_or_else(|_| {
                logger.error(format!("Cannot create a directory at {}" , config_directory.display()).as_ref());

                // Exit process since it is no use to continue
                std::process::exit(0)
            });

            config_directory.join("takoyaki")
        }
    };

    create_dir_all(&endpoint).unwrap_or_else(|_| {
        logger.error(format!("Cannot create a directory at {}" , endpoint.display()).as_ref());

        // Exit process since it is no use to continue
        std::process::exit(0)
    });

    endpoint
}

pub fn download_file(url: &str , output_dir: PathBuf) {
    let logger = Logger::new();

    let mut command = Command::new("curl")
        .args([url , "--output" , output_dir.display().to_string().as_ref()])
        // Redirect the output to /dev/null 
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap_or_else(|_| {
            logger.error("Cannot run the `curl` command. Make sure that you have it installed!");

            std::process::exit(1)
        })
    ;

    command.wait().unwrap_or_else(|_| {
        logger.error("Error while downloading the file with `curl`. Make sure you have an active internet connections or report the bug here - https://report.kyeboard.me/takoyaki");

        std::process::exit(1)
    });
}

