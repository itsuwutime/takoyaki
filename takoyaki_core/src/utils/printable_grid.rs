use toml::{Value, value::Map};
use colored::*;

use crate::{TakoyakiConfig, Result, Error};

#[derive(Default , Clone , PartialEq , Debug)]
pub struct Printable<'a> {
    pub color: &'a str,
    pub contributions: usize
}

#[derive(Default , Clone , PartialEq , Debug)]
pub struct PrintableGrid<'a> {
    pub grid: Vec<Vec<Printable<'a>>>
}

#[derive(Default , Debug)]
pub struct Hex {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl<'a> PrintableGrid<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_at(&mut self , x: usize , y: usize , item: Printable<'a>) {
        // Resize the grid on the x axis with vec![] as a default fill value
        if self.grid.len() <= x {
            self.grid.resize(x + 1, vec![])
        }

        // Resize the grid on the y axis with PrintableGrid::default() as a default fill value
        if self.grid[x].len() <= y {
            self.grid[x].resize(y , Printable::default())
        }

        // Insert at the specific location
        self.grid[x].insert(y , item);
    }

    pub fn hex_to_rgb(&self , hex: &str) -> Result<Hex> {
        if hex.len() != 7 || !hex.starts_with("#") {
            return Err(Error::InvalidHexColorCode)
        }

        let r = u8::from_str_radix(&hex[1..3] , 16).map_err(|_| Error::InvalidHexColorCode)?;
        let g = u8::from_str_radix(&hex[3..5] , 16).map_err(|_| Error::InvalidHexColorCode)?;
        let b = u8::from_str_radix(&hex[5..7] , 16).map_err(|_| Error::InvalidHexColorCode)?;

        Ok(Hex {
            r,
            g,
            b
        })
    }

    pub fn hint_color(&self , color_scheme: &Map<String , Value> , count: usize , fallback: String) -> String {
        let preffered_color = color_scheme.get(&format!("{}_contribution" , count));

        match preffered_color {
            Some(color) => {
                color.as_str().unwrap().to_string()
            },
            None => {
                color_scheme.get("x_contribution")
                    .unwrap_or(&Value::String(fallback))
                    .as_str()
                    .unwrap()
                    .to_string()
            }
        }
    }

    pub fn pretty_print(&self , config: Option<TakoyakiConfig>) -> Result<()> {
        // Get the config
        let tconfig = config.unwrap_or(TakoyakiConfig::get()?);

        // Get all the color prefs
        let raw_colors = tconfig.colors.unwrap_or(Value::Table(Map::new()));

        let colors = raw_colors.as_table().unwrap();

        for row in &self.grid {
            for col in row {
                let color = self.hex_to_rgb(&self.hint_color(&colors, col.contributions, col.color.to_string()))?;

                print!("{}" , "ඞ ".truecolor(color.r , color.g , color.b));
            }

            println!()
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_test() {
        let mut grid = PrintableGrid::new();

        let p_1 = Printable::default();
        let p_2 = Printable::default();
        let p_3 = Printable::default();

        // Insert items
        grid.insert_at(0, 0, p_1.clone());
        grid.insert_at(1, 1, p_2.clone());
        grid.insert_at(1, 2, p_3.clone());

        assert_eq!(grid.grid[0][0] , p_1);
        assert_eq!(grid.grid[1][1] , p_2);
        assert_eq!(grid.grid[1][2] , p_3);
    }

    #[test]
    fn invalid_hex_len() {
        let grid = PrintableGrid::new();

        assert_eq!(grid.hex_to_rgb("#invalid_hex").unwrap_err() , Error::InvalidHexColorCode);
    }

    #[test]
    fn invalid_hex_but_perfect_len() {
        let grid = PrintableGrid::new();

        assert_eq!(grid.hex_to_rgb("#hahaha").unwrap_err() , Error::InvalidHexColorCode);
    }

    #[test]
    fn valid_hex() {
        let grid = PrintableGrid::new();

        let rgb = grid.hex_to_rgb("#88C0D0").unwrap();

        assert_eq!(rgb.r , 136);
        assert_eq!(rgb.g , 192);
        assert_eq!(rgb.b , 208);
    }

    #[test]
    fn hint_color_should_use_fallback_color() {
        let grid = PrintableGrid::new();
        let config = TakoyakiConfig::from_str(r#""#).unwrap();

        assert_eq!(grid.hint_color(
            config.colors.unwrap_or(Value::Table(Map::new())).as_table().unwrap(),
            10,
            "#88C0D0".to_string()
        ) , "#88C0D0");
    }

    #[test]
    fn hint_color_should_use_x_contribution_color() {
        let grid = PrintableGrid::new();
        let config = TakoyakiConfig::from_str(r#"
            [colors]
            x_contribution = '#88C0D0'
        "#).unwrap();

        assert_eq!(grid.hint_color(
            config.colors.unwrap_or(Value::Table(Map::new())).as_table().unwrap(),
            10,
            "#2E3440".to_string()
        ) , "#88C0D0");
    }

    #[test]
    fn hint_color_should_use_count_contribution_color() {
        let grid = PrintableGrid::new();
        let config = TakoyakiConfig::from_str(r#"
            [colors]
            6_contribution = '#88C0D0'
        "#).unwrap();

        assert_eq!(grid.hint_color(
            config.colors.unwrap_or(Value::Table(Map::new())).as_table().unwrap(),
            6,
            "#2E3440".to_string()
        ) , "#88C0D0");
    }

    #[test]
    fn pretty_print_should_exit_nicely() {
        let mut grid = PrintableGrid::new();

        let p_1 = Printable::default();
        let p_2 = Printable::default();
        let p_3 = Printable::default();

        // Insert items
        grid.insert_at(0, 0, p_1.clone());
        grid.insert_at(1, 1, p_2.clone());
        grid.insert_at(1, 2, p_3.clone());

        assert!(grid.pretty_print(None).is_ok());
    }
}

