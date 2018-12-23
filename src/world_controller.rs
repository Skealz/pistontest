//! World controller.

use piston::input::GenericEvent;
use World;
use std::time::{SystemTime, Duration};

/// Handles the game events
pub struct WorldController<'c>
{
    ///Stores the world state
    pub world: World<'c>,

    /// Previous time the world has been updated
    pub previous_time : SystemTime

}

impl<'c> WorldController<'c>
{

    ///Creates a new WorldController
    pub fn new(world: World) -> WorldController
    {
        WorldController
        {
            world: world,
            previous_time: SystemTime::now()
        }
    }

    /// Returns the world
    pub fn get_world(&self) -> &World<'c>
    {
        &self.world
    }

    ///Handles Events
    pub fn events(&'c mut self)
    {
        //println!("Canard");
        if self.previous_time.elapsed().unwrap().as_secs() > 0 || self.previous_time.elapsed().unwrap().subsec_millis() > 900
        {
            self.world.update();
            self.previous_time = SystemTime::now();
        }
    }
}
