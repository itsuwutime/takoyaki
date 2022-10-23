use colored::*;

use crate::config::{Config, ConfigType};

#[derive(Debug)]
pub struct Printable {
    pub color: String,
    pub count: usize
}

#[derive(Debug , Default)]
pub struct PrintableGrid {
    grid: Vec<Vec<Printable>>
}


impl PrintableGrid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self , x: usize, _y: usize , item: Printable) {
        if self.grid.len() == x {
            self.grid.push(vec![])
        }

        self.grid[x].push(item)
    }

    pub fn pretty_print(&self , config: Config) {
        let prefs = config.get();

        for row in self.grid.iter() {
            for col in row {
                let raw_color = self.get_color(col.count , prefs , col.color.clone());

                let color = colorsys::Rgb::from_hex_str(raw_color.as_ref()).unwrap();

                if prefs.unicode.paint == "bg" {
                    print!("{}" , prefs.unicode.unicode.on_truecolor(color.red() as u8 , color.green() as u8 , color.blue() as u8))
                } else {
                    print!("{}" , prefs.unicode.unicode.truecolor(color.red() as u8 , color.green() as u8 , color.blue() as u8))
                }

            }
            println!()
        }
    }

    pub fn get_color(&self , count: usize , config: &ConfigType , fallback: String) -> String {
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
                return colors.get("any_contributions").unwrap_or(&toml::Value::String(fallback)).as_str().unwrap().to_string()
            }
        }
    }
}

