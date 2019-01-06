use graphics::types::Color;

/// Stores the settings for rendering and interaction.
#[derive(Clone)]
pub struct Settings {
    /// Whether or not the grid should wrap around, i.e. cells at the
    /// edge of the grid should have neighbors on the opposite edge.
    pub wraparound: bool,

    /// Distance of grid from the top-left corner of the window.
    pub offset: f64,
    /// Width of each cell.
    pub cell_width: f64,
    /// Distance between each cell.
    pub cell_distance: f64,

    /// Color of the background.
    pub background_color: Color,
    /// Color of live cells.
    pub live_color: Color,
    /// Color of dead cells.
    pub dead_color: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            wraparound: true,

            offset: 4.0,
            cell_width: 16.0,
            cell_distance: 2.0,

            background_color: [0.4, 0.4, 0.4, 1.0],
            live_color: [0.5, 0.8, 1.0, 1.0],
            dead_color: [0.1, 0.1, 0.2, 1.0],
        }
    }
}
