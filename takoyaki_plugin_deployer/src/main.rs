mod deploy;
mod utils;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/"  ,routes![deploy::create_new_deployment])
}

