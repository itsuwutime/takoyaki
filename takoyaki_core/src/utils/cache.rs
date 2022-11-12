use std::{path::PathBuf, io::Write, fs::File};
use crate::{hint_cache_path , Result , Error};
use serde::Deserialize;

#[derive(Clone)]
pub struct Cache {
    cache_file: PathBuf
}

impl Cache {
    pub fn new(app_name: &str) -> Result<Self> {
        Ok(Self {
            cache_file: hint_cache_path(app_name)?
        })
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self {
            cache_file: path
        }
    }

    pub fn validate(&self) -> bool {
        self.cache_file.is_file()
    }

    fn create_parent_folder(&self) -> Result<()> {
        // Create the parent folder
        std::fs::create_dir_all(
            self.cache_file.parent().ok_or(Error::PathWithNoParent)?
        ).map_err(|_| Error::CannotCreateDirectory)?;

        // Ok! :)
        Ok(())
    }

    pub fn create(&self) -> Result<()> {
        // Create parent folder
        self.create_parent_folder()?;

        // Write some empty string to create that file
        self.write_as_str("")?;

        Ok(())
    }

    pub fn write_as_str(&self , data: &str) -> Result<()> {
        // Create file file
        let mut file = File::create(&self.cache_file).map_err(|_| Error::CannotCreateFile)?;

        // Write to the file
        file.write_all(data.as_bytes()).map_err(|_| Error::CannotWriteToFile)?;

        // Ok! :)
        Ok(())
    }

    pub fn retrieve<T>(&self) -> Result<T>
    where 
        T: for<'de> Deserialize<'de>
    {
        let raw = std::fs::read_to_string(&self.cache_file).map_err(|_| Error::CannotCreateFile)?;

        let parsed = serde_json::from_str(&raw).map_err(|_| Error::SerializationError)?;

        Ok(parsed)
    }

    pub fn exists(&self) -> bool {
        self.cache_file.exists()
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::prelude::PermissionsExt;

    use super::*;

    #[test]
    pub fn cache_should_not_exist() {
        let cache = Cache::from_path(PathBuf::from("/some/random/path/"));

        assert_eq!(cache.exists() , false)
    }

    #[test]
    pub fn cache_file_should_be_created() {
        let mut cache_path = PathBuf::new();

        // Extend to the temp folder
        cache_path.extend([".temp" , "cache.json"]);

        // Build Cache from path
        let cache = Cache::from_path(cache_path);

        // Create file 
        assert!(cache.create().is_ok());

        // Create cache
        assert_eq!(cache.exists() , true)
    }

    #[test]
    pub fn cache_file_with_no_parent() {
        let cache_path = PathBuf::from("/");

        // Build Cache from path
        let cache = Cache::from_path(cache_path);

        // Create cache
        assert_eq!(cache.create().unwrap_err() , Error::PathWithNoParent)
    }

    #[test]
    pub fn cache_path_which_requires_root() {
        let cache_path = PathBuf::from("/root/cache.json");

        // Build Cache from path
        let cache = Cache::from_path(cache_path);

        // Create cache
        assert_eq!(cache.create().unwrap_err() , Error::CannotCreateFile)
    }

    #[test]
    pub fn cache_path_which_requires_root_with_parent() {
        let cache_path = PathBuf::from("/root/some_parent/cache.json");

        // Build Cache from path
        let cache = Cache::from_path(cache_path);

        // Create cache
        assert_eq!(cache.create().unwrap_err() , Error::CannotCreateDirectory)
    }

    #[test]
    pub fn cache_path_should_be_written() {
        let mut cache_path = PathBuf::new();

        // Extend to the temp folder
        cache_path.extend([".temp" , "valid_cache.json"]);

        // Build Cache from path
        let cache = Cache::from_path(cache_path.clone());

        // Create cache
        assert!(cache.create().is_ok());

        // Write some random string 
        assert!(cache.write_as_str("UwU").is_ok());

        // Check if the content is same
        assert_eq!(std::fs::read_to_string(cache_path).unwrap() , "UwU")
    }

    #[test]
    pub fn cache_path_cannot_be_written_because_of_permissions() {
        let mut cache_path = PathBuf::new();

        // Extend to the temp folder
        cache_path.extend([".temp" , "read_only_cache.json"]);

        // Build Cache from path
        let cache = Cache::from_path(cache_path.clone());

        // Create cache
        assert!(cache.create().is_ok());

        // Set read-only permissions
        std::fs::set_permissions(&cache_path, std::fs::Permissions::from_mode(0o555)).unwrap();

        // Write some random string 
        assert_eq!(cache.write_as_str("UwU").unwrap_err() , Error::CannotCreateFile)
    }

    #[test]
    pub fn invalid_json_cache_should_error() {
        let mut cache_path = PathBuf::new();

        // Extend to the temp folder
        cache_path.extend([".temp" , "cache.json"]);

        // Build Cache from path
        let cache = Cache::from_path(cache_path.clone());

        // Create cache
        assert!(cache.create().is_ok());

        // Write some random string 
        assert!(cache.write_as_str(r#"{ some_bugged_json UwU }"#).is_ok());

        assert_eq!(cache.retrieve::<serde_json::Value>().unwrap_err() , Error::SerializationError);
    }

    #[test]
    pub fn valid_json_cache_should_not_error() {
        let mut cache_path = PathBuf::new();

        // Extend to the temp folder
        cache_path.extend([".temp" , "another_cache.json"]);

        // Build Cache from path
        let cache = Cache::from_path(cache_path.clone());

        // Create cache
        assert!(cache.create().is_ok());

        // Write some random string 
        assert!(cache.write_as_str(r#"{ "mood": "UwU" }"#).is_ok());

        assert!(cache.retrieve::<serde_json::Value>().is_ok());
    }
}

