#[macro_use] extern crate rocket;
mod middlewares;
mod route;
mod utils;

// Get CORS fairing triggered
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[launch]
fn rocket() -> _ {
    // Create a new setup
    let setup = utils::Setup::new();

    // Setup directories
    setup.setup().unwrap();

    rocket::build()
        .attach(utils::Cors)
        .mount("/" , routes![route::create_new_deployment , route::poll_logs, all_options])
}
