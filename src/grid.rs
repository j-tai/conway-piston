use std::ops::Index;
use std::ops::IndexMut;

/// A Game of Life grid state.
#[derive(Clone, Default)]
pub struct Grid {
    cells: Vec<Vec<bool>>,
}

impl Grid {
    /// Create an empty grid with all cells dead.
    pub fn new(rows: usize, cols: usize) -> Grid {
        Grid { cells: vec![vec![false; cols]; rows] }
    }

    /// Get the number of rows in this grid.
    pub fn rows(&self) -> usize { self.cells.len() }

    /// Get the number of columns in this grid.
    pub fn cols(&self) -> usize {
        if self.rows() == 0 { 0 } else { self.cells[0].len() }
    }

    /// Get the cell at the given position. If the position is out of
    /// bounds, return `false`.
    pub fn get(&self, row: usize, col: usize) -> bool {
        if row >= self.rows() || col >= self.cols() {
            false
        } else {
            self[row][col]
        }
    }

    /// Clear the grid, setting all cells to dead.
    pub fn clear(&mut self) {
        let rows = self.rows();
        let cols = self.cols();
        self.cells = vec![vec![false; cols]; rows];
    }

    /// Resize the grid. Any cells that become out of bounds are
    /// removed, and new space is filled with dead cells. The top-left
    /// cell remains in the same place.
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.cells.resize(rows, vec![false; cols]);
        for row in &mut self.cells {
            row.resize(cols, false);
        }
    }

    /// Count the number of alive neighbors of a cell.
    fn count_neighbors(&self, r: usize, c: usize) -> usize {
        let mut count = 0;
        let l = r == 0;
        let t = c == 0;
        if !l && !t && self.get(r-1, c-1) { count += 1; }
        if !l && self.get(r-1, c  ) { count += 1; }
        if !l && self.get(r-1, c+1) { count += 1; }
        if !t && self.get(r  , c-1) { count += 1; }
        if !t && self.get(r+1, c-1) { count += 1; }
        if self.get(r+1, c  ) { count += 1; }
        if self.get(r  , c+1) { count += 1; }
        if self.get(r+1, c+1) { count += 1; }
        count
    }

    /// Step to the next generation in-place.
    pub fn step(&mut self) {
        let rows = self.rows();
        let cols = self.cols();
        let mut new_cells = vec![vec![false; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                let count = self.count_neighbors(r, c);
                new_cells[r][c] = match (self[r][c], count) {
                    (false, 3) => true,
                    (true, 2) => true,
                    (true, 3) => true,
                    _ => false
                };
            }
        }
        self.cells = new_cells;
    }
}

impl Index<usize> for Grid {
    type Output = [bool];
    fn index(&self, r: usize) -> &[bool] { &self.cells[r] }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, r: usize) -> &mut [bool] { &mut self.cells[r] }
}
