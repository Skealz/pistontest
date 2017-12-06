#![deny(missing_docs)]

//! Learning to use Piston

extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::input::RenderEvent;

pub use gameboard::Gameboard;
mod gameboard;

fn main() {
    let opengl = OpenGL::V3_2;

    /// Contains the settings for the window that will be created after.
    let settings = WindowSettings::new("Learning Piston", [500,500])
    .exit_on_esc(true).opengl(opengl);

    /// Creating the window
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    /// Creating the event loop with event settings
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    /// Event loop. events.next return the an Event item for the current loop.
    /// this loop
    while let Some(e) = events.next(&mut window)
    {
        ///     e : Event is a enum that contains the differents events
        if let Some(args) = e.render_args()
        {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([0.0,0.0,0.0,1.0], g);
            });
        }
    }
}
