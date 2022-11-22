use rocket::serde::json::Json;
use crate::utils::create_deployment;
use serde::Deserialize;

#[derive(Deserialize , Debug)]
#[serde(crate = "rocket::serde")]
pub struct DeployData {
    pub name: String,
    pub github_url: String,
    pub branch: String,
    pub path: String
}

#[post("/deploy" , format="application/json" , data="<data>")]
pub fn create_new_deployment(data: Json<DeployData>) {
    let uuid = uuid::Uuid::new_v4().to_string();

    tokio::spawn(
        create_deployment(
            uuid, 
            data.name.clone(),
            data.github_url.clone(), 
            data.branch.clone(), 
            data.path.clone()
        )
    );
}
