use serde::Deserialize;
use toml::Value;
use toml::map::Map;
use crate::{Error , build_path};


#[derive(Deserialize , Debug , Default)]
pub struct TakoyakiConfig {
    pub unicode: Unicode,
    pub colors: Map<String , Value>
}

#[derive(Deserialize , Debug , Default)]
pub struct Unicode {
    pub unicode: String,
    pub paint: String,
    pub fg_on_bg: Option<String>
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

        let parsed: TakoyakiConfig = toml::from_str(&raw).unwrap();

        Ok(Self {
            config: parsed
        })
    }
}
