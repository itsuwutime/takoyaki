use std::path::PathBuf;
use serde::Deserialize;

use crate::{Result , hint_config_path, Error};

pub struct Config {
    config_path: PathBuf
}

impl Config {
    pub fn new(name: &str) -> Result<Self> {
        Ok(Self {
            config_path: hint_config_path(name)?
        })
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self {
            config_path: path
        }
    }

    pub fn retrieve<T>(&self) -> Result<T> 
    where
        T: for<'de> Deserialize<'de>
    {
        let raw = std::fs::read_to_string(&self.config_path).map_err(|_| Error::CannotReadFile)?;

        toml::from_str(&raw).map_err(|_| Error::SerializationTOMLError)
    }

    pub fn exists(&self) -> bool {
        self.config_path.exists()
    }
}

#[cfg(test)]
mod test {
    use std::fs::{create_dir_all, File};
    use std::io::Write;

    use super::*;

    #[test] 
    fn config_should_not_exist() {
        let config = Config::from_path(PathBuf::from("/some/random/path"));

        assert_eq!(config.exists() , false)
    }

    #[test]
    fn bugged_config_should_error_out() {
        // Setup bugged config
        let config_path = PathBuf::new().join(".temp").join("bugged_config.toml");
        let bugged_config = "kekwkwkwkwk UwU Some UWU Contents UWUW"; // Uh this is a serious bugged config

        assert!(create_dir_all(config_path.parent().unwrap()).is_ok());

        let mut file = File::create(&config_path).unwrap();
        file.write_all(bugged_config.as_bytes()).unwrap();

        let config = Config::from_path(config_path);

        assert_eq!(config.retrieve::<toml::Value>().unwrap_err() , Error::SerializationTOMLError)
    }

    #[test]
    fn valid_config_should_not_error_out() {
        // Setup bugged config
        let config_path = PathBuf::new().join(".temp").join("config.toml");
        let bugged_config = r#"mood = "UwU" "#; // Uh this is a serious bugged config

        // Create parent directory
        assert!(create_dir_all(config_path.parent().unwrap()).is_ok());

        // Write config
        let mut file = File::create(&config_path).unwrap();
        file.write_all(bugged_config.as_bytes()).unwrap();

        // Create Config class
        let config = Config::from_path(config_path);

        // Struct that represents the config
        #[derive(Deserialize)]
        pub struct TConfig {
            pub mood: String
        }

        let parsed = config.retrieve::<TConfig>().unwrap();

        assert_eq!(&parsed.mood , "UwU");
    }
}

