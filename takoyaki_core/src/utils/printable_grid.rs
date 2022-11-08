use colorsys::Rgb;
use serde::Serialize;
use colored::*;

#[derive(Debug , Default , Clone , Serialize)]
pub struct Printable {
    pub color: String,
    pub contribution_count: usize
}

#[derive(Debug , Default , Serialize)]
pub struct PrintableGrid {
    grid: Vec<Vec<Printable>>
}

impl PrintableGrid {
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

    pub fn convert_to_rgb(&self , color: &String) -> Rgb {
        colorsys::Rgb::from_hex_str(color).unwrap()
    }

    pub fn pretty_print(&self) {
        for row in &self.grid {
            for col in row {
                let color = self.convert_to_rgb(&col.color);

                print!("{}" , "ඞ ".truecolor(color.red() as u8 , color.green() as u8 , color.blue() as u8));
            }
            println!();
        }
    }
}

