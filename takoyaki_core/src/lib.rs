mod utils;
mod test_utils;
mod takoyaki;
mod errors;

// Reexport 
pub use utils::*;
pub use takoyaki::*;
pub use test_utils::*;
pub use errors::*;

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
        let cache = Cache::new(PathBuf::from("/some/path/that/cannot/be/created"));

        assert!(cache.create().is_err());
        assert_eq!(cache.exists() , false);
    }

    #[test]
    fn no_ready_function() {
        let takoyaki = Takoyaki::new("new_plug");
        let response = takoyaki.start();

        assert!(matches!(response.unwrap_err() , Errors::NoStartFunctionFound));
    }

    #[test]
    fn no_execute_function() {
        let mut takoyaki = Takoyaki::new("new_plug");

        takoyaki.set_ready(Box::new(|| {  }));

        let response = takoyaki.start();

        assert!(matches!(response.unwrap_err() , Errors::NoExecuteFunctionFound));
    }

    #[test]
    fn should_be_ok() {
        let mut takoyaki = Takoyaki::new("new_plug");

        takoyaki.set_ready(Box::new(|| {  }));
        takoyaki.set_execute(Box::new(|| {  }));

        let response = takoyaki.start();

        assert!(response.is_ok());
    }
}

