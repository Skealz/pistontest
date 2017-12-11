//! World

use Organism;
use constants::SIZE;

/// Stores game board information
pub struct World
{
    /// Stores all the organisms
    pub organisms : Vec<Organism>,
}

impl World
{
    /// Creates a new world
    pub fn new() -> World
    {
        World
        {
            organisms : Vec::new(),
        }
    }

    /// Adds a new organism
    pub fn add_organism(&mut self, organism : Organism)
    {
        self.organisms.push(organism);
    }

    /// Returns a 2d map containing the occupied coordinates (marked as a true bool) and the free ones
    pub fn get_map_usage(&self) -> Vec<Vec<bool>>
    {
        //let mut map_usage = vec![vec![false; SIZE]; SIZE];
        let mut map_usage : Vec<Vec<bool>> = vec![vec![false; SIZE]; SIZE];
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
