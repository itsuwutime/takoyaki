mod utils;
mod takoyaki;

// Reexport 
pub use utils::*;
pub use takoyaki::*;

#[cfg(test)]
mod tests {
    mod utils;
    use std::path::PathBuf;

    use serde::Deserialize;
    use serde_json::Value;

    use super::*;

    // # # # # # # # # # # # # # # # #
    // #                             #
    // #       Printable Grid        #
    // #                             #
    // # # # # # # # # # # # # # # # #
    #[test]
    fn insert_printable() {
        let mut printable = PrintableGrid::new();
        let p_1 = Printable { color: "orange".to_string() , contribution_count: 10 };
        let p_2 = Printable { color: "blue".to_string() , contribution_count: 1 };
        let p_3 = Printable { color: "red".to_string() , contribution_count: 5 };

        printable.insert(1 , 1 , p_1.clone());
        printable.insert(1 , 4 , p_2.clone());
        printable.insert(2 , 2 , p_3.clone());

        assert_eq!(printable.grid[1][1] , p_1);
        assert_eq!(printable.grid[1][4] , p_2);
        assert_eq!(printable.grid[2][2] , p_3);
    }

    #[test]
    fn convert_to_rbg_should_fail() {
        let printable = PrintableGrid::new();

        let rgb = printable.convert_to_rgb(Some("random_hex_code_that_is_not_even_a_hex".to_string()));

        assert!(rgb.is_err());
    }

    #[test]
    fn convert_to_rbg_should_not_fail() {
        let printable = PrintableGrid::new();

        let rgb = printable.convert_to_rgb(Some("#88C0D0".to_string()));

        assert!(rgb.is_ok());

        let color = rgb.unwrap();

        assert_eq!(color.red() , 136.0);
        assert_eq!(color.green() , 192.0);
        assert_eq!(color.blue() , 208.0);
    }

    #[test]
    fn hint_color_should_fallback() {
        let printable = PrintableGrid::new();

        let rgb = printable.hint_color(&TakoyakiConfig::default(), 10, "#88C0D0");

        assert_eq!(rgb.unwrap() , "#88C0D0");
    }

    #[test]
    fn hint_color_should_use_x_color() {
        let printable = PrintableGrid::new();

        let sample_config = r#"
            [colors]
            x_contribution = '#88C0D0'
        "#; 

        let rgb = printable.hint_color(&TConfig::from_str(sample_config).unwrap().config, 10, "UwU");

        assert_eq!(rgb.unwrap() , "#88C0D0");
    }

    #[test]
    fn hint_color_should_use_defined_color() {
        let printable = PrintableGrid::new();

        let sample_config = r#"
            [colors]
            2_contribution = '#88C0D0'
        "#; 

        let rgb = printable.hint_color(&TConfig::from_str(sample_config).unwrap().config, 2, "UwU");

        assert_eq!(rgb.unwrap() , "#88C0D0");
    }

    // # # # # # # # # # # # # # # # #
    // #                             #
    // #           Config            #
    // #                             #
    // # # # # # # # # # # # # # # # #
    #[test]
    fn invalid_config_parse_should_fail() {
        let sample_config = r#"
            [colors]
            <uwuwuw> ^-^ UWU
        "#; 

        let config = TConfig::from_str(sample_config);

        assert!(config.is_err())
    }

    #[test]
    fn invalid_config_parse_should_not_fail() {
        let sample_config = r#"
            [colors]
            x_contribution = '#88C0D0'
        "#; 

        let config = TConfig::from_str(sample_config);

        assert!(config.is_ok());
        assert!(config.unwrap().config.unicode.is_none())
    }

    // # # # # # # # # # # # # # # # #
    // #                             #
    // #          Cache              #
    // #                             #
    // # # # # # # # # # # # # # # # #
    #[test]
    fn cache_should_not_exist() {
        let cache = Cache::from_path(PathBuf::from("/some/random/path/uwu"));

        assert_eq!(cache.exists() , false)
    }

    #[test]
    fn cache_create_should_fail() {
        let cache = Cache::from_path(PathBuf::from("/some/random/path/uwu"));

        assert!(cache.create().is_err())
    }

    #[test]
    fn cache_create_should_create_new_dir() {
        let cache = Cache::from_path(PathBuf::from(".temp").join("cache.json"));

        assert!(cache.create().is_ok());
        assert!(cache.exists());
    }

    #[test]
    fn cache_should_not_be_retrievable() {
        let cache = Cache::from_path(PathBuf::from("/some/random/path/uwu"));

        assert!(cache.retrieve::<Value>().is_err())
    }

    #[test]
    fn cache_should_be_writable() {
        let cache = Cache::from_path(PathBuf::from(".temp").join("cache.json"));

        assert!(cache.create().is_ok());

        cache.populate_as_str(r#"{ "mood": "UwU" }"#).unwrap();
    }

    // # # # # # # # # # # # # # # # #
    // #                             #
    // #          State              #
    // #                             #
    // # # # # # # # # # # # # # # # #
    #[tokio::test]
    async fn state_without_anything() {
        let state = ReadyState::empty();

        assert!(state.resolve::<Value>().await.is_err());
    }

    #[tokio::test]
    async fn state_from_cache() {
        let cache = Cache::from_path(PathBuf::from(".temp").join("cache.json"));

        #[derive(Deserialize)]
        struct Response {
            mood: String
        }

        // Write some basic cache
        assert!(cache.populate_as_str(r#"{ "mood": "UwU" }"#).is_ok());

        let state = ReadyState::from_cache(cache);

        let data = state.resolve::<Response>().await.unwrap();

        assert_eq!(data.mood , "UwU");
    }
}

