//! World

extern crate rand;

use Organism;
use constants::WORLD_SIZE;
use rand::Rng;
use piston::window::Position;
use constants::FOOD_PROP;
use rusttype::{Point,point};

/// Stores game board information
pub struct World
{
    /// Stores all the organisms
    pub organisms : Vec<Organism>,

    /// Food position on the map
    pub food : Vec<Point<usize>>
}

impl World
{
    /// Creates a new world
    pub fn new() -> World
    {
        let mut food : Vec<Point<usize>> = Vec::new();
        let nb_food = ((WORLD_SIZE * WORLD_SIZE) as f32 * FOOD_PROP).trunc() as i32;
        for i in 0..nb_food
        {
            let mut x = rand::thread_rng().gen_range(0, WORLD_SIZE);
            let mut y = rand::thread_rng().gen_range(0, WORLD_SIZE);
            let mut p : Point<usize> = point(x, y);

            while (food.contains(&p))
            {
                x = rand::thread_rng().gen_range(0, WORLD_SIZE);
                y = rand::thread_rng().gen_range(0, WORLD_SIZE);
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

    /// Returns food position
    pub fn get_food_pos(&self) -> &Vec<Point<usize>>
    {
        return &self.food;
    }

    /// Creates initial organisms
    pub fn create_initial_orgs(&mut self)
    {
        for i in 0..10
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
                map_usage[cell.position.x][cell.position.y] = true;
            }
        }
        map_usage
    }


}
