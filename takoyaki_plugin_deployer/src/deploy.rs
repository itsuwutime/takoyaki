use crate::utils::Deployer;

#[post("/deploy")]
pub fn create_new_deployment() -> String {
    let deployer = Deployer::create_new_deployment();

    serde_json::to_string_pretty(&deployer.identifier).unwrap()
}

