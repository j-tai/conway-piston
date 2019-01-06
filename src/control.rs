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
    /// Whether the mouse is currently drawing live or dead
    /// cells. `None` if the mouse button is not being held.
    drawing_state: Option<bool>,
}

impl Control {
    /// Create a new `Control`.
    pub fn new(grid: Grid) -> Control {
        Control {
            grid,
            hover_cell: None,
            cursor_pos: (0.0, 0.0),
            drawing_state: None,
        }
    }

    /// Handle an event.
    pub fn event<E: GenericEvent>(&mut self, set: &Settings, e: &E) {
        // Handle window resize, resizing the grid accordingly
        if let Some([x, y]) = e.resize_args() {
            let rows = (y - set.offset * 2.0 - set.cell_distance) / (set.cell_width + set.cell_distance);
            let cols = (x - set.offset * 2.0 - set.cell_distance) / (set.cell_width + set.cell_distance);
            let rows = if rows <= 1.0 { 1 } else { rows as usize };
            let cols = if cols <= 1.0 { 1 } else { cols as usize };
            self.grid.resize(rows, cols);
        }

        // Handle hovered cell
        if let Some([x, y]) = e.mouse_cursor_args() {
            self.cursor_pos = (x, y);
            let off = set.offset + set.cell_distance / 2.0;
            let cell_off = set.cell_width + set.cell_distance;
            let hover_x = ((self.cursor_pos.0 - off) / cell_off) as isize;
            let hover_y = ((self.cursor_pos.1 - off) / cell_off) as isize;
            if hover_x < 0 || hover_y < 0 || hover_x >= self.grid.cols() as isize || hover_y >= self.grid.rows() as isize {
                self.hover_cell = None;
            } else {
                let cell = (hover_y as usize, hover_x as usize);
                if self.hover_cell != Some(cell) {
                    // Mouse moved to a different cell
                    self.hover_cell = Some(cell);
                    if let Some(state) = self.drawing_state {
                        self.grid[cell] = state;
                    }
                }
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            if let Some(hover) = self.hover_cell {
                let state = !self.grid[hover];
                self.drawing_state = Some(state);
                self.grid[hover] = state;
            }
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            self.drawing_state = None;
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space {
                self.grid.step()
            }
        }
    }
}
