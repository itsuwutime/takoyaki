use colorsys::Rgb;
use serde::Serialize;
use colored::*;
use toml::Value;

use crate::{TConfig, TakoyakiConfig};

#[derive(Debug , Default , Clone , Serialize)]
pub struct Printable {
    pub color: String,
    pub contribution_count: usize
}

#[derive(Debug , Default , Serialize)]
pub struct PrintableGrid {
    grid: Vec<Vec<Printable>>
}

impl<'a> PrintableGrid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self , x: usize , y: usize , item: Printable) {
        if self.grid.len() <= x {
            self.grid.resize(x + 1, vec![])
        }

        if self.grid[x].len() <= y {
            self.grid[x].resize(y , Printable::default())
        }

        self.grid[x].insert(y, item);
    }

    pub fn convert_to_rgb(&self , color: &str) -> Rgb {
        colorsys::Rgb::from_hex_str(color).unwrap()
    }

    pub fn hint_color(&self , config: &TakoyakiConfig , count: usize , fallback: &'a str) -> String {
        // Get all the color
        let colors = config.colors.as_table().unwrap();

        // Get the specified color for the contribution count 
        let pref = colors.get(&format!("{}_contribution" , count));

        // Match pref
        match pref {
            Some(color) => {
                color.as_str().unwrap().to_string()
            },
            None => {
                colors.get("x_contribution")
                    .unwrap_or(&Value::String(fallback.to_owned()))
                    .as_str()
                    .unwrap()
                    .to_string()
            }
        }
    } 

    pub fn pretty_print(&self) {
        let user_prefs = TConfig::new().unwrap().config;

        for row in &self.grid {
            for col in row {
                let color = self.convert_to_rgb(&self.hint_color(&user_prefs , col.contribution_count , &col.color));

                if user_prefs.unicode.paint == "bg" {
                    let mut printable = "ඞ ".on_truecolor(color.red() as u8 , color.green() as u8 , color.blue() as u8);

                    if user_prefs.unicode.fg_on_bg.clone().is_some() {
                        let fg = self.convert_to_rgb(&user_prefs.unicode.fg_on_bg.clone().unwrap());

                        printable = printable
                            .truecolor(fg.red() as u8 , fg.blue() as u8 , fg.blue() as u8);
                    }

                    print!("{}" , printable);
                } else {
                    print!("{}" , "ඞ ".truecolor(color.red() as u8 , color.green() as u8 , color.blue() as u8));
                }

            }
            println!();
        }
    }
}

