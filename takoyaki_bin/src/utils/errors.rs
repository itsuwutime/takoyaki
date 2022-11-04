use super::Logger;

pub fn no_internet_connection(logger: &Logger) -> ! {
    logger.error("You don't have an active internet connection! Try connecting to a network.")
}

pub fn plugin_not_found(name: String , logger: &Logger) -> ! {
    logger.error(
        format!(
            "No plugin named {} found! If you think this is an error, please report it here - https://www.github.com/kyeboard/takoyaki/issues/new",
            name
        ).as_ref()
    );
}
