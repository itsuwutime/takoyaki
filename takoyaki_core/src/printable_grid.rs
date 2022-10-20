use std::str::FromStr;
use colored::*;

#[derive(Debug)]
pub struct Printable {
    pub color: String
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

    pub fn insert(&mut self , x: usize, y: usize , item: Printable) {
        if self.grid.len() == x {
            self.grid.push(vec![])
        }

        self.grid[x].push(item)
    }

    pub fn pretty_print(&self) {
        for row in self.grid.iter() {
            for col in row {
                let color = colorsys::Rgb::from_hex_str(col.color.as_ref()).unwrap();

                print!("{}" , "⬛".truecolor(color.red() as u8 , color.green() as u8 , color.blue() as u8))
            }
            println!()
        }
    }
}

