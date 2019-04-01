//! World

extern crate rand;

use Organism;
use constants::WORLD_SIZE;
use rand::Rng;
use constants::FOOD_PROP;
use rusttype::{Point,point};

/// Stores game board information
pub struct World
{
    /// Stores all the organisms
    pub organisms : Vec<Organism>,

    /// Food position on the map
    pub food : Vec<Point<i32>>
}

impl World
{
    /// Creates a new world
    pub fn new() -> World
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
    pub fn update(&mut self) -> Vec<usize>
    {
        let mut dead_orgs : Vec<usize> = Vec::new();
        for i in 0..self.organisms.len()
        {
            self.organisms[i].update_hunger();
            let hunger = self.organisms[i].get_hunger();
            if hunger <= 0
            {
                self.organisms[i].starving();
                self.organisms[i].update_life();
                self.organisms[i].remove_dead_cells();
                if self.organisms[i].get_cells().len() == 0
                {
                    dead_orgs.push(i);
                }
                //let index = self.organisms.binary_search(org);
                //let index = self.organisms.iter().position(|x| x == org).unwrap();
                //self.organisms.remove(index);
            }

            let mut food_aim = point(-1, -1);
            let mut closest_food_cell = point(-1, -1);
            let p = point(-1, -1);
            let has_food = self.organisms[i].update_closest_food(&self.food, &mut food_aim, &mut closest_food_cell);
            //println!("{:?} {:?}", food_aim, closest_food_cell);
            // If no food is found we need to send (-1,-1) to the moving function to tell
            // that it must take the generated tempdirection
            if !has_food
            {
                self.organisms[i].set_temp_dir(&food_aim);
                food_aim = point(-1, -1);
            }
            else
            {
                // Resetting the temp direction
                self.organisms[i].set_temp_dir(&p);
            }
            let ate = self.organisms[i].moving(&food_aim, &closest_food_cell);
            if ate
            {
                //Resetting the temp direction
                self.organisms[i].set_temp_dir(&p);
            }
            if ate && has_food
            {
                let index = self.food.iter().position(|&r| r == food_aim).unwrap();
                self.food.remove(index);
                self.organisms[i].eating();
                self.organisms[i].update_perception_movement();
                self.organisms[i].update_perception_area();
            }
        }

        dead_orgs
    }

    /// Remove the cells
    pub fn remove_dead_orgs(&mut self, indexes : Vec<usize>)
    {
        for index in indexes
        {
            self.organisms.remove(index);
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
