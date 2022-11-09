use serde::Deserialize;
use toml::Value;
use crate::{Error , build_path};

#[derive(Deserialize , Debug)]
pub struct TakoyakiConfig {
    pub unicode: Unicode,
    pub colors: Value
}

#[derive(Deserialize , Debug)]
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
