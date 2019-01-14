//! World view.

use graphics::types::Color;
use graphics::{Context,Graphics};
use constants::*;
use WorldController;
use World;
use Organism;
use graphics::{Rectangle};
use rusttype::{Point, point};

///Stores world view settings
pub struct WorldViewSettings
{
    /// Width of the Window
    pub width : i32,

    /// Height of the Window
    pub height : i32,

    /// Game background color
    pub background_color: Color,

    /// Game border color
    pub border_color : Color,

    ///Position from top-left corner
    pub view_position : Point<i32>,

    /// Define which part of the world is displayed
    pub view_width : i32,

    /// Define which part of the world is displayed
    pub view_height : i32
}

impl WorldViewSettings
{
    /// Creates a new World view settings
    pub fn new() -> WorldViewSettings
    {
        WorldViewSettings
        {
            view_position: point(0,0),
            width: WIN_WIDTH as i32,
            height : WIN_HEIGHT as i32,
            view_width : WORLD_SIZE as i32,
            view_height : WORLD_SIZE as i32,
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
      let board_rect = [0.0, 0.0, self.settings.width as f64, self.settings.height as f64];

      //Draw board background
      Rectangle::new(self.settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

      //self.draw_perceptions(controller, c, g);
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
              let mut pos_x: i32 =  cell.position.x as i32 - self.settings.view_position.x;
              let mut pos_y: i32 =  cell.position.y as i32 - self.settings.view_position.y;
              if (cell.position.x as i32) > self.settings.view_position.x && (cell.position.y as i32) > self.settings.view_position.y &&
               (cell.position.x as i32) < self.settings.view_position.x + self.settings.view_width &&
               (cell.position.y as i32) < self.settings.view_position.y + self.settings.view_height
              {
                  let zoom_factor: f64 = WIN_WIDTH as f64 / (self.settings.view_width) as f64;
                  pos_x = (pos_x as f64 * zoom_factor).trunc() as i32;
                  pos_y = (pos_y as f64 * zoom_factor).trunc() as i32;
                  //println!("{:?}", zoom_factor);
                  Rectangle::new([0.0, 255.0, 0.0, 1.0]).draw([pos_x as f64, pos_y as f64, zoom_factor.trunc() as f64, zoom_factor.trunc() as f64],
                       &c.draw_state, c.transform, g);
              }
          }
        }
    }

    /// Draw perceptions circles
    pub fn draw_perceptions<G: Graphics>(&self, controller: &WorldController, c: &Context, g: &mut G)
    {
        let world : &World = controller.get_world();
        let orgs : &Vec<Organism> = world.get_organisms();

        for org in orgs
        {
            let area_points = org.get_perception_area();

            for pnt in area_points
            {
                let mut pos_x: i32 =  pnt.x as i32 - self.settings.view_position.x;
                let mut pos_y: i32 =  pnt.y as i32 - self.settings.view_position.y;
                if (pnt.x as i32) > self.settings.view_position.x && (pnt.y as i32) > self.settings.view_position.y &&
                 (pnt.x as i32) < self.settings.view_position.x + self.settings.view_width &&
                 (pnt.y as i32) < self.settings.view_position.y + self.settings.view_height
                {
                    let zoom_factor: f64 = WIN_WIDTH as f64 / (self.settings.view_width) as f64;
                    pos_x = (pos_x as f64 * zoom_factor).trunc() as i32;
                    pos_y = (pos_y as f64 * zoom_factor).trunc() as i32;
                    Rectangle::new([0.0, 102.0, 255.0, 1.0]).draw([pos_x as f64, pos_y as f64, zoom_factor.trunc() as f64, zoom_factor.trunc() as f64],
                         &c.draw_state, c.transform, g);
                }
            }
        }

    }

    /// Draws food
    pub fn draw_food<G: Graphics>(&self, controller: &WorldController, c: &Context, g: &mut G)
    {
        let world : &World = controller.get_world();
        let food_pos : &Vec<Point<i32>> = world.get_food_pos();

        for pos in food_pos
        {
            let mut pos_x: i32 = pos.x as i32 - self.settings.view_position.x;
            let mut pos_y: i32 =  pos.y as i32 - self.settings.view_position.y;
            if (pos.x as i32) > self.settings.view_position.x && (pos.y as i32) > self.settings.view_position.y &&
             (pos.x as i32) < self.settings.view_position.x + self.settings.view_width && (pos.y as i32) < self.settings.view_position.y + self.settings.view_height
             {
                let zoom_factor: f64 = WIN_WIDTH as f64 / (self.settings.view_width) as f64;
                pos_x = (pos_x as f64 * zoom_factor).trunc() as i32;
                pos_y = (pos_y as f64 * zoom_factor).trunc() as i32;
                Rectangle::new([255.0, 0.0, 0.0, 1.0]).draw([pos_x as f64, pos_y as f64, zoom_factor.trunc() as f64, zoom_factor.trunc() as f64],
                     &c.draw_state, c.transform, g);
             }
        }
    }

    /// Change world view
    pub fn set_view(&mut self, pos: Point<i32>, width: i32, height: i32)
    {
        self.settings.view_position.x += ((self.settings.view_width as f64 / WIN_WIDTH) * pos.x as f64).trunc() as i32;
        self.settings.view_position.y += ((self.settings.view_width as f64 / WIN_WIDTH) * pos.y as f64).trunc() as i32;
        self.settings.view_width = ((self.settings.view_width as f64 / WIN_WIDTH) * width as f64).trunc() as i32;
        self.settings.view_height = ((self.settings.view_height as f64 / WIN_HEIGHT) * width as f64).trunc() as i32;
    }

    /// Reset world view to fit world size
    pub fn reset_view(&mut self)
    {
        self.settings.view_position.x = 0;
        self.settings.view_position.y = 0;
        self.settings.view_width = WORLD_SIZE as i32;
        self.settings.view_height = WORLD_SIZE as i32;
    }
}
