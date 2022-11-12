use std::path::PathBuf;
use crate::{Result , Error};

pub fn hint_cache_path(name: &str) -> Result<PathBuf> {
    // Get the configured cache directory
    let assumed_cache_dir = dirs::cache_dir();

    // Check if the cache directory is set
    let mut cache_dir = match assumed_cache_dir {
        Some(cache_dir) => {
            // If there is some, return it 
            cache_dir
        },
        None => {
            // if not, we are gonna redirect to $HOME/.cache
            let mut home_dir = dirs::home_dir().ok_or(Error::HomeDirectoryNotFound)?;

            // Extend till the $HOME/.cache
            home_dir.extend([".cache"]);

            home_dir
        }
    };

    // Extend till the cache path for the plugin
    cache_dir.extend(["takoyaki" , name , "cache.json"]);

    Ok(cache_dir)
}

pub fn hint_takoyaki_config_path() -> Result<PathBuf> {
    // Get the configured cache directory
    let assumed_config_dir = dirs::config_dir();

    // Check if the cache directory is set
    let mut config_dir = match assumed_config_dir {
        Some(config_dir) => {
            // If there is some, return it 
            config_dir
        },
        None => {
            // if not, we are gonna redirect to $HOME/.cache
            let mut home_dir = dirs::home_dir().ok_or(Error::HomeDirectoryNotFound)?;

            // Extend till the $HOME/.cache
            home_dir.extend([".config"]);

            home_dir
        }
    };

    // Extend till the cache path for the plugin
    config_dir.extend(["takoyaki", "config.toml"]);

    Ok(config_dir)
}

pub fn hint_config_path(name: &str) -> Result<PathBuf> {
    // Get the configured cache directory
    let assumed_config_dir = dirs::config_dir();

    // Check if the cache directory is set
    let mut config_dir = match assumed_config_dir {
        Some(config_dir) => {
            // If there is some, return it 
            config_dir
        },
        None => {
            // if not, we are gonna redirect to $HOME/.cache
            let mut home_dir = dirs::home_dir().ok_or(Error::HomeDirectoryNotFound)?;

            // Extend till the $HOME/.cache
            home_dir.extend([".config"]);

            home_dir
        }
    };

    // Extend till the cache path for the plugin
    config_dir.extend(["takoyaki" , name , "config.toml"]);

    Ok(config_dir)
}

#[cfg(test)]
mod test {
    use crate::Result;
    use super::*;

    #[test]
    fn should_be_okay_when_home_is_set() -> Result<()> {
        let path = hint_cache_path("anyplug")?;
        let mut intended_path = dirs::cache_dir().unwrap();

        intended_path.extend(["takoyaki" , "anyplug" , "cache.json"]);

        assert_eq!(path , intended_path);

        Ok(())
    }

    #[test]
    fn should_be_okay_even_when_home_is_not_set() -> Result<()> {
        // Unset $HOME
        std::env::remove_var("HOME");

        let path = hint_cache_path("anyplug")?;
        let mut intended_path = dirs::home_dir().unwrap();

        intended_path.extend([".cache" , "takoyaki" , "anyplug" , "cache.json"]);

        assert!(std::env::var("HOME").is_err());
        assert_eq!(path , intended_path);

        Ok(())
    }

    #[test]
    fn config_should_be_okay_when_home_is_set() -> Result<()> {
        let path = hint_config_path("anyplug")?;
        let mut intended_path = dirs::config_dir().unwrap();

        intended_path.extend(["takoyaki" , "anyplug" , "config.toml"]);

        assert_eq!(path , intended_path);

        Ok(())
    }

    #[test]
    fn config_should_be_okay_even_when_home_is_not_set() -> Result<()> {
        // Unset $HOME
        std::env::remove_var("HOME");

        let path = hint_config_path("anyplug")?;
        let mut intended_path = dirs::home_dir().unwrap();

        intended_path.extend([".config" , "takoyaki" , "anyplug" , "config.toml"]);

        assert!(std::env::var("HOME").is_err());
        assert_eq!(path , intended_path);

        Ok(())
    }
}
