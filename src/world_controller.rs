//! World controller.

use piston::input::GenericEvent;
use World;

/// Handles the game events
pub struct WorldController
{
    ///Stores the world state
    pub world: World,
}

impl WorldController
{

    ///Creates a new WorldController
    pub fn new(world: World) -> WorldController
    {
        WorldController
        {
            world: world,
        }
    }

    /// Returns the world
    pub fn get_world(&self) -> &World
    {
        &self.world
    }

    ///Handles Events
    pub fn events<E: GenericEvent>(&mut self, e: &E)
    {

    }
}
