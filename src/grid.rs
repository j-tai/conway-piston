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

    fn get_isize(&self, mut row: isize, mut col: isize, wrap: bool) -> bool {
        if wrap {
            if row < 0 {
                row += self.rows() as isize;
            } else {
                row %= self.rows() as isize;
            }
            if col < 0 {
                col += self.cols() as isize;
            } else {
                col %= self.cols() as isize;
            }
        } else if row < 0 || col < 0 {
            return false;
        }
        self.get(row as usize, col as usize)
    }

    /// Count the number of alive neighbors of a cell.
    fn count_neighbors(&self, r: usize, c: usize, wrap: bool) -> usize {
        let r = r as isize;
        let c = c as isize;
        let mut count = 0;
        for nr in r-1 ..= r+1 {
            for nc in c-1 ..= c+1 {
                // Don't count the same cell
                if (nr, nc) == (r, c) { continue; }
                if self.get_isize(nr, nc, wrap) { count += 1; }
            }
        }
        count
    }

    /// Step to the next generation in-place. If `wrap` is `true`,
    /// then the grid will wrap around, i.e. cells at the edges will
    /// have neighbors on the opposite side of the grid.
    pub fn step(&mut self, wrap: bool) {
        let rows = self.rows();
        let cols = self.cols();
        let mut new_cells = vec![vec![false; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                let count = self.count_neighbors(r, c, wrap);
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

impl Index<(usize, usize)> for Grid {
    type Output = bool;
    fn index(&self, (r, c): (usize, usize)) -> &bool { &self.cells[r][c] }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut bool { &mut self.cells[r][c] }
}
