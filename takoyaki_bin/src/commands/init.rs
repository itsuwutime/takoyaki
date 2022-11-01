use crate::utils::{Logger , get_config_directory , download_file};

pub fn initialize_instance() {
    // Create new instance of logger and get the config dir
    let logger = Logger::new();
    let config = get_config_directory();

    // Create config path
    let config_path = config.join("config.toml");

    // A bit of success messages
    logger.success("Initializing a new instance of takoyaki...");
    logger.success(format!("Populating default config for takoyaki at {}" , config_path.display()).as_ref());

    // Download the file
    download_file("https://raw.githubusercontent.com/kyeboard/takoyaki/main/default.toml" , config_path);

    // Success ^-^
    logger.success("Successfully populated default config!");
    logger.success("Done!");
}

