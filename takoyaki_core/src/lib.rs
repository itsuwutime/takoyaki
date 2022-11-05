mod utils;
mod test_utils;

// Reexport 
pub use utils::*;
pub use test_utils::*;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use super::test_utils::test_utils::*;

    #[test]
    fn cache_path_does_not_exit() {
        let cache = Cache::new(PathBuf::from("SOME_RANDOM_SH*T"));

        assert_eq!(cache.exists() , false);
    }

    #[test]
    fn cache_path_created_and_exists() {
        let cache = Cache::new(cache_dir());

        assert!(cache.create().is_ok());
        assert!(cache.exists());
    }

    #[test]
    fn cache_folder_create_error() {
        let cache = Cache::new(PathBuf::from("/path/that/will/probably/never/be/able/to/be/created/yeah"));

        assert!(cache.create().is_err());
        assert_eq!(cache.exists() , false);
    }
}

