//! World view.

use graphics::types::Color;
use graphics::{Context,Graphics};

use WorldController;

///Stores world view settings
pub struct WorldViewSettings
{
    ///Position from top-left corner
    pub position : [f64;2],

    ///Size of the world
    pub size : f64,

    /// Game background color
    pub background_color: Color,

    /// Game border color
    pub border_color : Color,
}

impl WorldViewSettings
{
    /// Creates a new World view settings
    pub fn new() -> WorldViewSettings
    {
        WorldViewSettings
        {
            position: [50.0; 2],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
        }
    }
}

/// Stores visual information about a world
pub struct WorldView
{
    ///Stores world view settings
    pub settings : WorldViewSettings,
}

impl WorldView
{
    /// Creates a new world view
    pub fn new(settings : WorldViewSettings) -> WorldView
    {
        WorldView
        {
            settings : settings,
        }
    }

    /// Draw world
    pub fn draw<G: Graphics>(&self, controller: &WorldController, c: &Context, g: &mut G)
    {
      use graphics::{Line, Rectangle};

      let board_rect = [self.settings.position[0], self.settings.position[1], self.settings.size, self.settings.size];

      //Draw board background
      Rectangle::new(self.settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);
    }
}
