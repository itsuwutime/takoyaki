#[macro_use] extern crate lazy_static;
pub use parking_lot::Mutex;

mod utils;
mod middleware;
mod commands;

// Reexport
pub use utils::*;
pub use commands::*;

// lazy load static items
lazy_static! {
    static ref LOGGER: Logger = Logger::new();
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

#[tokio::main]
async fn main() {
    setup();

    let server = Server::new().leak();

    server.listen(3000).await.unwrap();
}

