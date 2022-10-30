// Colored library to colorize the outputs
use anyhow::Result;
use colored::*;
use colorsys::Rgb;

// Use in built modules
use crate::config::{Config, ConfigType};

#[derive(Debug , Clone)]
pub enum State {
    Empty,
    Some
}

impl Default for State {
    fn default() -> Self {
        Self::Empty
    }
}

// Printable struct
#[derive(Debug , Default , Clone)]
pub struct Printable {
    pub color: String,
    pub count: usize,
    pub state: State
}

// Main grid
#[derive(Debug , Default)]
pub struct PrintableGrid {
    grid: Vec<Vec<Printable>>
}

// Functions 
impl PrintableGrid {
    pub fn new() -> Self {
        Self::default()
    }

    // Inserts a printable at `x` and `y` coords
    pub fn insert(&mut self , x: usize, y: usize , item: Printable) {
        // Resize the grid till `x` on x axis
        if self.grid.len() <= x {
            self.grid.resize(x + 1, vec![]);
        }

        // Resize the grid till `y` ony axis
        self.grid[x].resize(y , Printable::default());

        // Insert the printable
        self.grid[x].insert(y , item);
    }

    pub fn pretty_print(&self , config: Config) -> Result<()> {
        // Get parsed config
        let prefs = config.get();

        // Iterate through all the rows
        for row in self.grid.iter() {
            // Iterate through all the columns
            for col in row {
                // Get the color to be used
                let raw_color = self.get_color(&col.count , prefs , &col.color);

                // Split hex to rgb
                let color = Rgb::from_hex_str(raw_color.as_ref())?;

                // Print according to the preference
                if prefs.unicode.paint == "bg" {
                    print!(
                        "{}" , prefs.unicode.unicode.on_truecolor(
                            color.red() as u8,
                            color.green() as u8,
                            color.blue() as u8
                        )
                    )
                } else { 
                    print!(
                        "{}" , prefs.unicode.unicode.truecolor(
                            color.red() as u8,
                            color.green() as u8,
                            color.blue() as u8
                        )
                    )
                }

            }
            println!()
        }

        Ok(())
    }

    pub fn get_color(&self , count: &usize , config: &ConfigType , fallback: &String) -> String {
        // Get all the colors as a HashMap
        let colors = config.colors.as_table().unwrap();

        // Get the specific color for the contribution count
        let color = colors.get(&format!("{}_contribution" , count));

        // Check color
        match color {
            // The color for the number of contributions exists!
            Some(hex) => {
                // Return that color
                return hex.as_str().unwrap_or_else(|| panic!("Invalid color for `{}_contributions!`" , count)).to_string()
            },
            None => {
                // Use `any_contributions` color or fallback to the original color
                return colors.get("any_contributions").unwrap_or(&toml::Value::String(fallback.to_owned())).as_str().unwrap().to_string()
            }
        }
    }
}

