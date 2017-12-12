//! World view.

use graphics::types::Color;
use graphics::{Context,Graphics};
use constants::*;
use WorldController;
use World;
use Cell;
use Organism;

///Stores world view settings
pub struct WorldViewSettings
{
    ///Position from top-left corner
    pub position : [f64;2],

    /// Width of the view
    pub width : f64,

    /// Height of the view
    pub height : f64,

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
            position: [0.0; 2],
            width: WIN_WIDTH,
            height : WIN_HEIGHT,
            background_color: [0.0, 0.0, 0.0, 1.0],
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

      let board_rect = [self.settings.position[0], self.settings.position[1], self.settings.width, self.settings.height];

      //Draw board background
      Rectangle::new(self.settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

      let world : &World = controller.get_world();

      let orgs : &Vec<Organism> = world.get_organisms();
      for org in orgs
      {
        let cells = org.get_cells();
        for cell in cells
        {
            Rectangle::new([0.0, 255.0, 0.0, 1.0]).draw([cell.position.x as f64, cell.position.y as f64, 1.0, 1.0],
                 &c.draw_state, c.transform, g);
        }
      }
    }
}
