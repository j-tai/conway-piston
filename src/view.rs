use graphics::Context;
use graphics::Graphics;
use graphics::Rectangle;

use crate::Control;
use crate::Settings;

/// Render the screen.
pub fn draw<G: Graphics>(ctl: &Control, set: &Settings, c: &Context, g: &mut G) {
    let off = set.offset + set.cell_distance;
    let cell_off = set.cell_width + set.cell_distance;
    let rows = ctl.grid.rows();
    let cols = ctl.grid.cols();

    // Grid background
    Rectangle::new(set.background_color)
        .draw([set.offset, set.offset,
               set.cell_distance + (cols as f64) * cell_off,
               set.cell_distance + (rows as f64) * cell_off],
              &c.draw_state, c.transform, g);

    // Cells
    let live = Rectangle::new(set.live_color);
    let dead = Rectangle::new(set.dead_color);
    for row in 0..rows {
        for col in 0..cols {
            let x = off + (col as f64) * cell_off;
            let y = off + (row as f64) * cell_off;
            let rect = if ctl.grid[row][col] { live } else { dead };
            rect.draw([x, y, set.cell_width, set.cell_width],
                      &c.draw_state, c.transform, g);
        }
    }
}
