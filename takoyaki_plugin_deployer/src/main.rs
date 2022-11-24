#[macro_use] extern crate rocket;
mod middlewares;
mod route;
mod utils;

#[launch]
fn rocket() -> _ {
    let setup = utils::Setup::new();

    setup.setup().unwrap();

    rocket::build()
        .mount("/" , routes![route::create_new_deployment , route::poll_logs])
}

