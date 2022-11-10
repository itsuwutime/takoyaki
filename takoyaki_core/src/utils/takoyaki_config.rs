use serde::Deserialize;
use toml::Value;
use toml::map::Map;
use crate::{Error , build_path};


#[derive(Deserialize , Debug , Default , Clone)]
pub struct TakoyakiConfig {
    pub unicode: Option<Unicode>,
    pub colors: Option<Map<String , Value>>
}

#[derive(Deserialize , Debug , Clone)]
pub struct Unicode {
    pub unicode: Option<String>,
    pub paint: Option<String>,
    pub fg_on_bg: Option<String>
}

impl Default for Unicode {
    fn default() -> Self {
        Self {
            unicode: Some("ඞ ".to_string()),
            paint: Some("fg".to_string()),
            fg_on_bg: None
        }
    }
}

pub struct TConfig {
    pub config: TakoyakiConfig
}

impl TConfig {
    pub fn new() -> Result<Self , Error> {
        let raw = std::fs::read_to_string(
            build_path()
                .map_err(|_| Error::ConfigDirNotFound)?
                .join("config.toml")
        ).map_err(|_| Error::ReadError)?;

        let parsed: TakoyakiConfig = toml::from_str(&raw).map_err(Error::SerializeTOMLError)?;

        Ok(Self {
            config: parsed
        })
    }

    pub fn from_str(raw: &str) -> Result<Self , Error> {
        Ok(Self {
            config: toml::from_str(raw).map_err(Error::SerializeTOMLError)?
        })
    }
}

