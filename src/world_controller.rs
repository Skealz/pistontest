//! World controller.

use piston::input::GenericEvent;
use World;
use std::time::{SystemTime, Duration};

/// Handles the game events
pub struct WorldController
{
    ///Stores the world state
    pub world: World,

    /// Previous time the world has been updated
    pub previous_time : SystemTime

}

impl WorldController
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
    pub fn get_world(&self) -> &World
    {
        &self.world
    }

    ///Handles Events
    pub fn events(&mut self)
    {
        //println!("Canard");
        if self.previous_time.elapsed().unwrap().as_secs() > 0 || self.previous_time.elapsed().unwrap().subsec_millis() > 500
        {
            let dead_orgs = self.world.update();
            self.world.remove_dead_orgs(dead_orgs);
            self.previous_time = SystemTime::now();
        }
    }
}
