use colored::*;

use crate::config::Config;

#[derive(Debug)]
pub struct Printable {
    pub color: String,
    pub count: usize
}

#[derive(Debug)]
pub struct PrintableGrid {
    grid: Vec<Vec<Printable>>
}

impl PrintableGrid {
    pub fn new() -> Self {
        Self {
            grid: vec![]
        }
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
                let raw_color = self.get_color(col.count , &config);

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

    pub fn get_color(&self , count: usize , config: &Config) -> String {
        let prefs = config.get();

        println!("{}" , prefs.colors.as_table().unwrap().get("1_contribution").unwrap().as_str().unwrap());

        String::new()
    }
}

