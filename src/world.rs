//! World

extern crate rand;

use Organism;
use constants::WORLD_SIZE;
use rand::Rng;
use constants::FOOD_PROP;
use rusttype::{Point,point};

/// Stores game board information
pub struct World<'b>
{
    /// Stores all the organisms
    pub organisms : Vec<Organism<'b>>,

    /// Food position on the map
    pub food : Vec<Point<i32>>
}

impl<'b> World<'b>
{
    /// Creates a new world
    pub fn new() -> World<'b>
    {
        let mut food : Vec<Point<i32>> = Vec::new();
        let nb_food = ((WORLD_SIZE * WORLD_SIZE) as f32 * FOOD_PROP).trunc() as i32;
        for _i in 0..nb_food
        {
            let mut x = rand::thread_rng().gen_range(0, WORLD_SIZE) as i32;
            let mut y = rand::thread_rng().gen_range(0, WORLD_SIZE) as i32;
            let mut p : Point<i32> = point(x, y);

            while food.contains(&p)
            {
                x = rand::thread_rng().gen_range(0, WORLD_SIZE) as i32;
                y = rand::thread_rng().gen_range(0, WORLD_SIZE) as i32;
                p = point(x, y);
            }

            food.push(point(x,y));
        }

        World
        {
            organisms : Vec::new(),
            food : food,
        }
    }

    /// Updates the world
    pub fn update(&mut self)
    {
        for org in self.organisms.iter_mut()
        {
            org.update(&self.food);
        }
    }

    /// Returns food position
    pub fn get_food_pos(&self) -> &Vec<Point<i32>>
    {
        return &self.food;
    }

    /// Creates initial organisms
    pub fn create_initial_orgs(&mut self)
    {
        for _i in 0..1
        {
            let map_usage = self.get_map_usage();
            self.organisms.push(Organism::new(map_usage));
        }
    }

    /// Returns organisms
    pub fn get_organisms(&self) -> &Vec<Organism>
    {
        &self.organisms
    }

    /// Adds a new organism
    pub fn add_organism(&mut self, organism : Organism)
    {
        self.organisms.push(organism);
    }

    /// Returns a 2D map containing the occupied coordinates (marked as a true bool) and the free ones
    pub fn get_map_usage(&self) -> Vec<Vec<bool>>
    {
        //let mut map_usage = vec![vec![false; WORLD_SIZE]; WORLD_SIZE];
        let mut map_usage : Vec<Vec<bool>> = vec![vec![false; WORLD_SIZE]; WORLD_SIZE];
        for org in self.organisms.iter()
        {
            let cells = org.get_cells();
            for cell in cells.iter()
            {
                map_usage[cell.position.x as usize][cell.position.y as usize] = true;
            }
        }
        map_usage
    }


}
