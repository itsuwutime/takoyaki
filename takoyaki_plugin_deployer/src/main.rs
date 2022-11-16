#[macro_use]
extern crate lazy_static;

mod server;
mod logger;
mod builder;
mod message;
mod state;

use parking_lot::Mutex;
// Reimport
pub use server::*;
pub use logger::*;
pub use builder::*;
pub use message::*;
pub use state::*;

// Lazy load state
lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

#[tokio::main]
async fn main() {
    let server = Box::leak(Box::new(Server::new()));
    let port = option_env!("PORT").unwrap_or("3000");

    let logger = Logger::new();

    logger.success(&format!("Starting server on port {}..." , port));

    server.start(
        vec!["HAHHAHAHA"]
    ).await;
}

