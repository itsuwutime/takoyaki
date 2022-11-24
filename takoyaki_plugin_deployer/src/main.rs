// Import dependencies
#[macro_use]
extern crate rocket;

use std::error::Error;

mod middlewares;
mod route;
mod utils;

// Get CORS fairing triggered
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new setup
    let setup = utils::Setup::instance();

    // Setup directories
    setup.setup()?;

    // New Rocket!
    let _ = rocket::build()
        .attach(utils::Cors)
        .mount("/", routes![route::create_new_deployment, route::poll_logs, all_options])
        .launch()
        .await?;

    // Ok!
    Ok(())
}
