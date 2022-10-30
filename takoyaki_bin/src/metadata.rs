use reqwest::Result;
use serde::{Deserialize , Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub repository: String,
    pub description: String,
    pub developer_url: String,
    pub image: String,
    pub home_page: String,
    pub config_url: String
}

#[derive(Deserialize)]
pub struct Response {
    plugins: Vec<Metadata>
}

pub async fn get_metadata(name: &str) -> Result<Option<Metadata>> {
    let plugins: Response = reqwest::get("https://raw.githubusercontent.com/kyeboard/takoyaki/main/plugins/plugins.json").await?.json().await?;

    for plugin in plugins.plugins.iter() {
        if plugin.name.to_lowercase() == name.to_string() {
            return Ok(Some(plugin.clone()))
        }
    };

    Ok(None)
}
