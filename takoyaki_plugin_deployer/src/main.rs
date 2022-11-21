#[macro_use] extern crate rocket;
mod route;
mod middlewares;

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/", routes![route::create_new_deployment])
}
