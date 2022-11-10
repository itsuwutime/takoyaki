mod utils;
mod takoyaki;

// Reexport 
pub use utils::*;
pub use takoyaki::*;

#[cfg(test)]
mod tests {
    mod utils;
    use super::*;
    use toml::value::Map;
    use toml::Value::{self , Table};

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
        let mut tconfig = TakoyakiConfig::default();

        tconfig.colors.insert("colors".to_string(), Table(Map::new()));
        tconfig.colors
            .get::<String>(&"colors".to_string()).unwrap()
            .as_table().unwrap()
            .insert("colors".to_string(), Table(Map::new()));

        let rgb = printable.hint_color(&TakoyakiConfig::default(), 10, "#88C0D0");

        assert_eq!(rgb.unwrap() , "#88C0D0");
    }

}

