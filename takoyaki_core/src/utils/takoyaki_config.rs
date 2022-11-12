use serde::Deserialize;
use toml::Value;
use crate::{Result, hint_takoyaki_config_path, Error};

#[derive(Deserialize)]
pub struct Unicode {
    pub unicode: Option<String>,
    pub bg: Option<String>,
    pub fg_on_bg: Option<String>
}

#[derive(Deserialize)]
pub struct TakoyakiConfig {
    pub unicode: Option<Unicode>,
    pub colors: Option<Value>
}

impl TakoyakiConfig {
    pub fn get() -> Result<Self> {
        let config_path = hint_takoyaki_config_path()?;

        let raw = std::fs::read_to_string(&config_path).map_err(|_| Error::ConfigNotFound)?;

        toml::from_str(&raw).map_err(|_| Error::SerializationTOMLError)
    }

    pub fn from_str(raw: &str) -> Result<Self> {
        toml::from_str(&raw).map_err(|_| Error::SerializationTOMLError)
    }
}

