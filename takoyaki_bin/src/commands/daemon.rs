use crate::utils::Logger;

pub fn start_daemon() {
    let logger = Logger::new();

    logger.success("Starting daemon");

    // Infinite loop
    loop {
        // Refresh plugins
        crate::commands::refresh();

        logger.success("Finished refreshing... Sleeping for an hour...");

        // Sleep for an hour to prevent update on every second (thats mad)
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
