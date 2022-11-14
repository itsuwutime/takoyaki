mod deploy;
mod error;
mod setup;
mod utils;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    setup::setup().unwrap();

    rocket::build()
        .mount("/", routes![deploy::create_new_deployment])
}
