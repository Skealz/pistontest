//! World view.

use graphics::types::Color;
use graphics::{Context,Graphics};
use constants::*;
use WorldController;
use World;
use Organism;
use graphics::{Rectangle};
use rusttype::Point;

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
      let board_rect = [self.settings.position[0], self.settings.position[1], self.settings.width, self.settings.height];

      //Draw board background
      Rectangle::new(self.settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

      self.draw_cells(controller, c, g);
      self.draw_food(controller, c, g);
    }

    /// Draws cells
    pub fn draw_cells<G: Graphics>(&self, controller: &WorldController, c: &Context, g: &mut G)
    {
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

    /// Draws food
    pub fn draw_food<G: Graphics>(&self, controller: &WorldController, c: &Context, g: &mut G)
    {
        let world : &World = controller.get_world();
        let food_pos : &Vec<Point<usize>> = world.get_food_pos();

        for pos in food_pos
        {
            Rectangle::new([255.0, 0.0, 0.0, 1.0]).draw([pos.x as f64, pos.y as f64, 1.0, 1.0],
                 &c.draw_state, c.transform, g);
        }
    }
}
