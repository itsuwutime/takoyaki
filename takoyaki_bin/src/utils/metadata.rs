use reqwest::Result;
use serde::{Deserialize , Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub repository: String,
    pub description: String,
    pub developer_url: String,
    pub image: String,
    pub home_page: String,
    pub config_url: String
}

#[derive(Deserialize , Clone)]
pub struct Response {
    plugins: Vec<Metadata>
}

pub async fn get_metadata(name: &str) -> Result<Option<Metadata>> {
    let plugins: Response = reqwest::get("https://raw.githubusercontent.com/kyeboard/takoyaki/main/plugins/plugins.json").await?.json().await?;

    Ok(plugins.plugins.into_iter().find(|plugin| {
        plugin.name.to_lowercase() == *name
    }))
}

