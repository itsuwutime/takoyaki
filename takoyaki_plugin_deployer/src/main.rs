mod server;
mod logger;
mod builder;
mod message;

#[tokio::main]
async fn main() {
    let server = Box::leak(Box::new(server::Server::new()));
    let port = option_env!("PORT").unwrap_or("3000");

    let logger = logger::Logger::new();

    logger.success(&format!("Starting server on port {}..." , port));

    server.start(
        vec!["HAHHAHAHA"]
    ).await;
}

