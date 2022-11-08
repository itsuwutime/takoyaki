use serde::Serialize;

#[derive(Debug , Default , Clone , Serialize)]
pub struct Printable<'a> {
    pub color: &'a str,
    pub contribution_count: u32
}

#[derive(Debug , Default , Serialize)]
pub struct PrintableGrid<'a> {
    grid: Vec<Vec<Printable<'a>>>
}

impl<'a> PrintableGrid<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self , x: usize , y: usize , item: Printable<'a>) {
        if self.grid.len() <= x {
            self.grid.resize(x + 1, vec![])
        }

        if self.grid[x].len() <= y {
            self.grid[x].resize(y , Printable::default())
        }

        self.grid[x].insert(y, item);
    }
}

