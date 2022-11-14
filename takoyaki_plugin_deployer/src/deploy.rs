use crate::utils::Deployer;
use rocket::serde::{json::Json, Deserialize};

// Request format
#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Plugin {
    pub name: String,
    pub github_url: String,
    pub branch: String,
    pub path: String,
}

#[post("/deploy", format = "application/json", data = "<plugin>")]
pub async fn create_new_deployment(plugin: Json<Plugin>) -> String {
    String::new()
}
