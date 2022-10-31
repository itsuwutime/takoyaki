use anyhow::Result;
use crate::logger::Logger;

pub fn start_daemon() -> Result<()> {
    let logger = Logger::new();

    logger.success("Starting daemon");

    // Infinite loop
    loop {
        logger.success("Refreshing cache...");

        // Refresh plugins
        crate::refresh::refresh_plugins()?;

        logger.success("Finished refreshing... Sleeping for an hour...");

        // Sleep for an hour to prevent update on every second (thats mad)
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
