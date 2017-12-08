//! Gameboard controller.

use piston::input::GenericEvent;

use Gameboard;

pub struct GameboardController
{
    ///Stores the gameboard state
    pub gameboard: Gamboard;
}

impl GameboardController
{
    ///Creates a new GameboardController
    pub fn new(gameboard: Gameboard) -> GameboardController
    {
        GameboardController
        {
            gameboard: gameboard,
        }
    }

    ///Handles Events
    pub fn events<E: GenericEvents>(&mut self, e: &E)
    {
        
    }
}
