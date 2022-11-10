use std::collections::HashMap;

use colorsys::Rgb;
use serde::Serialize;
use colored::*;
use toml::Value;

use crate::{TConfig, TakoyakiConfig , Error};

#[derive(Debug , Default , Clone , Serialize , PartialEq)]
pub struct Printable {
    pub color: String,
    pub contribution_count: usize
}

#[derive(Debug , Default , Serialize , PartialEq)]
pub struct PrintableGrid {
    pub grid: Vec<Vec<Printable>>
}

impl<'a> PrintableGrid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self , x: usize , y: usize , item: Printable) {
        // Extend the grid on the x axis
        if self.grid.len() <= x {
            self.grid.resize(x + 1, vec![])
        }

        // Extend the grid on the y axis
        if self.grid[x].len() <= y {
            self.grid[x].resize(y , Printable::default())
        }

        // Insert the item at the position
        self.grid[x].insert(y, item);
    }

    pub fn convert_to_rgb(&self , color: Option<String>) -> Result<Rgb , Error> {
        // Use colorsys to convert hex to rgb
        match color {
            Some(color) => {
                colorsys::Rgb::from_hex_str(&color).map_err(|e| Error::HexColorParseError(Some(e)))
            },
            None => {
                Err(Error::InvalidHexColorCode)
            }
        }
    }

    pub fn hint_color(&'a self , config: &'a TakoyakiConfig , count: usize , fallback: &'a str) -> Option<String> {
        // Get all the color
        let colors = config.colors.clone();

        // Get the specified color for the contribution count 
        let pref = colors.get(&format!("{}_contribution" , count));

        // Match pref
        match pref {
            Some(color) => {
                // Return the color
                Some(color.as_str().unwrap().to_string())
            },
            None => {
                Some(colors.get("x_contribution") // Get global color config
                    .unwrap_or(&Value::String(fallback.to_owned())) // Fallback to original color if not available
                    .as_str()
                    .unwrap()
                    .to_string()
                )
            }
        }
    } 

    pub fn pretty_print(&self) -> Result<() , Error> {
        // Get user configs
        let user_prefs = TConfig::new().unwrap().config;

        // Iterate through all the rows available
        for row in &self.grid {
            // Iterate through all the cols present
            for col in row {
                // Convert the color to rgb
                let color = self.convert_to_rgb(
                    self.hint_color(&user_prefs , col.contribution_count , &col.color)
                ).map_err(|_| Error::HexColorParseError(None))?;

                // Check if the user wants to paint bg or fg
                if user_prefs.unicode.paint == "bg" {
                    // Create a printable variable
                    let mut printable = "ඞ ".on_truecolor(
                        color.red() as u8, 
                        color.green() as u8, 
                        color.blue() as u8
                    );

                    // Chek if user wants to paint fg as well
                    if user_prefs.unicode.fg_on_bg.clone().is_some() {
                        // Get the rgb combination for the fg
                        let fg = self.convert_to_rgb(
                            Some(user_prefs.unicode.fg_on_bg.as_ref().unwrap().to_string())
                        )?;

                        // Add text color
                        printable = printable.truecolor(
                            fg.red() as u8, 
                            fg.blue() as u8, 
                            fg.blue() as u8
                        );
                    }

                    // Print the printable
                    print!("{}" , printable);
                } else {
                    print!("{}" , "ඞ ".truecolor(
                        color.red() as u8,
                        color.green() as u8,
                        color.blue() as u8)
                    );
                }

            }
            println!();
        }

        Ok(())
    }
}

