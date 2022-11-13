use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Deployer {
    pub identifier: DeployerIndentifier
}

#[derive(Serialize)]
pub struct DeployerIndentifier {
    pub uuid: String
}

impl Deployer {
    pub fn create_new_deployment() -> Self {
        Self {
            identifier: DeployerIndentifier { uuid: Uuid::new_v4().to_string() }
        }
    }
}

