mod server;
mod utils;

// Reexport
#[macro_use] extern crate lazy_static;

pub use server::*;
pub use utils::*;

// Result type
pub type Result<T> = std::result::Result<T , Error>;

// Lazy load
lazy_static! {
    static ref LOGGER: Logger = Logger::new();
}

#[tokio::main]
async fn main() {
    setup_workspace();

    let uuid = uuid::Uuid::new_v4().to_string();

    create_deployment("https://github.com/worldhellosdj/ddfdfdf.git", "main", "/", "new-plugin" , &uuid);

    // let server = Box::leak(Box::new(Server::new(3000)));
    //
    // server.listen().await;
}

