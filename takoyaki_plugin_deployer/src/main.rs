#[macro_use] extern crate rocket;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
mod middlewares;
mod route;
mod utils;

#[launch]
fn rocket() -> _ {
    // Create a new setup
    let setup = utils::Setup::new();

    // Setup directories
    setup.setup().unwrap();

    rocket::build()
        .mount("/" , routes![route::create_new_deployment , route::poll_logs])
}
