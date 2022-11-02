use crate::utils::{get_config_directory , Logger};

fn no_dir_error(logger: &Logger) -> ! {
    logger.error("Cannot find the directory! Make sure you have ran `takoyaki init`");

    std::process::exit(1)
}

pub fn refresh() {
    let config_directory = get_config_directory();
    let logger = Logger::new();

    logger.success("Refreshing cache...");

    // Iterate through the installed plugins and get the name
    for name in std::fs::read_dir(config_directory.join("plugins")).unwrap_or_else(|_| { no_dir_error(&logger) }) {
        let file_name = name.unwrap().file_name();

        // Remove the cache first
        std::fs::remove_dir_all(config_directory.join("cache").join(&file_name)).unwrap_or_else(|_| { no_dir_error(&logger) });

        // Use the plugin to fill up the new cache
        crate::commands::run(&file_name.to_str().unwrap().to_string() , true);
    }

    logger.success("Done!");
}
