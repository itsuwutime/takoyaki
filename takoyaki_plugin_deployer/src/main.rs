#[macro_use] extern crate rocket;
mod route;

#[launch]
fn launch() -> _ {
    rocket::build()
}
