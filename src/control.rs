use piston::input::Button;
use piston::input::GenericEvent;
use piston::input::Key;
use piston::input::MouseButton;

use crate::Grid;
use crate::Settings;

/// Handles events.
pub struct Control {
    /// The current state of the grid.
    pub grid: Grid,
    /// Position of the cell over which the mouse is hovering.
    pub hover_cell: Option<(usize, usize)>,
    /// Last mouse cursor position.
    cursor_pos: (f64, f64),
}

impl Control {
    /// Create a new `Control`.
    pub fn new(grid: Grid) -> Control {
        Control {
            grid,
            hover_cell: None,
            cursor_pos: (0.0, 0.0),
        }
    }

    /// Toggle the current hovered cell. If the mouse is not hovering
    /// over any cell, this method does nothing.
    pub fn toggle_hovered(&mut self) {
        if let Some((row, col)) = self.hover_cell {
            self.grid[row][col] = !self.grid[row][col];
        }
    }

    pub fn event<E: GenericEvent>(&mut self, set: &Settings, e: &E) {
        // Handle hovered cell
        if let Some([x, y]) = e.mouse_cursor_args() {
            self.cursor_pos = (x, y);
            let off = set.offset + set.cell_distance / 2.0;
            let cell_off = set.cell_width + set.cell_distance;
            let hover_x = ((self.cursor_pos.0 - off) / cell_off) as isize;
            let hover_y = ((self.cursor_pos.1 - off) / cell_off) as isize;
            if hover_x < 0 || hover_y < 0
                || hover_x >= self.grid.cols() as isize
                || hover_y >= self.grid.rows() as isize {
                self.hover_cell = None;
            } else {
                self.hover_cell = Some((hover_y as usize, hover_x as usize));
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            self.toggle_hovered();
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space {
                self.grid.step()
            }
        }
    }
}
