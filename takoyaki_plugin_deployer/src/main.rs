mod server;
mod message;

#[tokio::main]
async fn main() {
    let server = Box::leak(Box::new(server::Server::new()));

    server.start(
        vec!["HAHHAHAHA"]
    ).await;
}

