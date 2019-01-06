use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use opengl_graphics::GlGraphics;
use piston::event_loop::EventLoop;
use piston::event_loop::Events;
use piston::event_loop::EventSettings;
use piston::input::RenderEvent;
use piston::window::WindowSettings;

pub use crate::control::Control;
pub use crate::grid::Grid;
pub use crate::settings::Settings;
pub use crate::view::draw;

mod control;
mod grid;
mod settings;
mod view;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Conway's Game of Life", [442, 442])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let set = Settings::default();
    let grid = Grid::default();
    let mut ctl = Control::new(grid);

    while let Some(e) = events.next(&mut window) {
        ctl.event(&set, &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                graphics::clear([0.0, 0.0, 0.0, 1.0], g);
                draw(&ctl, &set, &c, g);
            });
        }
    }
}
