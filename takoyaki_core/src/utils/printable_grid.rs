#[derive(Default , Clone , PartialEq , Debug)]
pub struct Printable<'a> {
    pub color: &'a str,
    pub contributions: usize
}

#[derive(Default , Clone , PartialEq , Debug)]
pub struct PrintableGrid<'a> {
    pub grid: Vec<Vec<Printable<'a>>>
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
}
