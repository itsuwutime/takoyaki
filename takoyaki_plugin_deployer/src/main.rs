#[macro_use] extern crate rocket;
mod route;
mod utils;

#[launch]
fn rocket() -> _ {
    let setup = utils::Setup::new();

    setup.setup().unwrap();

    rocket::build()
        .mount("/" , routes![route::create_new_deployment])
}

