use rocket::serde::{json::Json , Deserialize};
use crate::middlewares::Auth;

#[derive(Deserialize , Debug)]
pub struct Deploy<'a> {
    pub github_url: &'a str,
    pub branch: &'a str,
    pub path: &'a str,
    pub name: &'a str
}

#[post("/deploy", format="application/json", data="<deploy>")]
pub fn create_new_deployment(deploy: Json<Deploy> , auth_guard: Auth) {

}

