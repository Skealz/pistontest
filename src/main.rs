#![deny(missing_docs)]
//! Learning to use Piston

extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;
extern crate rusttype;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::input::*;
use rusttype::{Point, point};
use std::time::{SystemTime, Duration};

pub use world::World;
pub use world_controller::WorldController;
pub use world_view::{WorldView, WorldViewSettings};
pub use organism::Organism;
pub use cell::Cell;
pub use constants::{WIN_WIDTH, WIN_HEIGHT};

mod world;
mod world_controller;
mod world_view;
mod cell;
mod organism;
mod constants;
mod func;

fn main() {
    let opengl = OpenGL::V3_2;

    // Contains the settings for the window that will be created after.
    let settings = WindowSettings::new("Learning Piston", [WIN_WIDTH.trunc() as u32, WIN_HEIGHT.trunc() as u32])
    .exit_on_esc(true).opengl(opengl);

    // Creating the window
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    // Creating the event loop with event settings
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut world = World::new();
    world.create_initial_orgs();
    let mut world_controller = WorldController::new(world);

    let world_view_settings : WorldViewSettings = WorldViewSettings::new();
    let mut world_view : WorldView = WorldView::new(world_view_settings);


    let mut cursor = point(0.0, 0.0);
    let mut pressed_pos = point(0.0,0.0);
    let mut release_pos = point(0.0,0.0);


    // Event loop. events.next return the an Event item for the current loop.
    // this loop
    while let Some(e) = events.next(&mut window)
    {
         //e : Event is an enum that contains the differents events
        if let Some(args) = e.render_args()
        {
            /*gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([0.0,0.0,0.0,1.0], g);
                world_view.draw(&world_controller, &c, g);
            });*/
        }
        if let Some(Button::Mouse(button)) = e.press_args() {
            if button == mouse::MouseButton::Left
            {
                pressed_pos = cursor;
                println!("Pressed mouse button '{:?}'", button);
            }
            else if button == mouse::MouseButton::Right
            {
                world_view.reset_view();
            }
        }
        if let Some(Button::Mouse(button)) = e.release_args() {
            if button == mouse::MouseButton::Left
            {
                release_pos = cursor;
                println!("Released mouse button '{:?}'", button);
                println!("Width {:?} Height {:?}", ((pressed_pos.x - release_pos.x) as f64).abs(), ((pressed_pos.y - release_pos.y) as f64).abs());
                let mut view_pos = point(0,0);
                if pressed_pos.x <= release_pos.x || pressed_pos.y <= release_pos.y
                {
                    view_pos = point((pressed_pos.x as f64).trunc() as i32, (pressed_pos.y as f64).trunc() as i32)
                }
                else
                {
                    view_pos = point((release_pos.x as f64).trunc() as i32, (release_pos.y as f64).trunc() as i32)
                }
                println!("PosX {:?} PosY {:?}", view_pos.x, view_pos.y);
                world_view.set_view(view_pos, ((pressed_pos.x - release_pos.x) as f64).abs() as i32, ((pressed_pos.y - release_pos.y) as f64).abs() as i32)
            }
        }
        e.mouse_cursor(|x, y| {
            cursor = point(x, y);
            //println!("Mouse moved '{} {}'", x, y);
        });

        if let Some(u) = e.update_args()
        {
            world_controller.events();
        }
    }
}
