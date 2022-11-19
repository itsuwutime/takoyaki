#[macro_use] extern crate lazy_static;

mod utils;
mod commands;

// Reexport
pub use utils::*;
pub use commands::*;

// lazy load static items
lazy_static! {
    static ref LOGGER: Logger = Logger::new();
}

#[tokio::main]
async fn main() {
    let server = Server::new().leak();

    server.listen(3000).await.unwrap();
}

