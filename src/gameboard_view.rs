//! Gameboard view.

use graphics::types::Color;
use graphics::{Context,Graphics};

use GameboardController;

///Stores gameboard view settings
pub struct GameboardViewSettings
{
    ///Position from top-left corner
    pub position : [f64;2],

    ///Size of the gameboard
    pub size : f64,

    pub background_color: color,

    pub border_color : color,
}

impl GameboardViewSettings
{
    pub fn new() -> GameboardViewSettings
    {
        position = [10.0,2],
        size = 400.0,
        background_color = [0.8, 0.8, 1.0, 1.0],
        border_color = [0.0, 0.0, 0.2, 1.0],
    }
}

pub struct GameboardView
{
    ///Stores gameboard view settings
    pub settings : GameboardViewSettings,
}

impl GameboardView
{
    pub fn new() -> GameboardView
    {
        GameboardView
        {
            settings : settings,
        }
    }

    pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G)
    {
        
    }
}
