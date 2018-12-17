//! Organism living in the world

extern crate rand;

use Cell;
use constants::WORLD_SIZE;
use rusttype::{Point, point};
use func;
use rand::Rng;

/// Living organism composed by cells.
pub struct Organism
{
    /// Cells that composes the organism
    cells : Vec<Cell>,

    /// Size of the hunder gauge
    hunger : i16,

    /// Current hunger of the oranism (between 0 and hunger)
    current_hunger : i16,

    /// Current perception ability
    current_perception : i16,

    /// Points defining the area in which the organism can see
    perception_area : Vec<Point<usize>>,
}

impl Organism
{
    /// Creates a new organism
    /// map_usage represents the current map usage.
    pub fn new(map_usage : Vec<Vec<bool>>) -> Organism
    {
        let mut cells : Vec<Cell> = Vec::new();

        let initial_shape = Organism::get_initial_shape();

        let mut chosed_pos = point(rand::thread_rng().gen_range(0, WORLD_SIZE), rand::thread_rng().gen_range(0, WORLD_SIZE));
        while !func::check_avail(&map_usage, chosed_pos, &initial_shape)
        {
            chosed_pos = point(rand::thread_rng().gen_range(0, WORLD_SIZE), rand::thread_rng().gen_range(0, WORLD_SIZE));
        }

        println!("{:?}", chosed_pos);

        for i in 0..initial_shape.len()
        {
            for j in 0..initial_shape[i].len()
            {
                if initial_shape[i][j]
                {
                    cells.push(Cell::new_random(point(chosed_pos.x + i, chosed_pos.y + j)));
                }
            }
        }

        let mut org = Organism
        {
            cells : cells,
            hunger : 10,
            current_hunger : 7,
            current_perception : 0,
            perception_area : Vec::new(),
        };
        org.update_perception();
        org
    }

    /// Updates the organism
    pub fn update(&mut self)
    {

    }

    /// Updates the area in which the organism can see. Uses current_perception value
    /// added to the cells in all the directions
    pub fn update_perception_area(&mut self)
    {
        let cells = &self.cells;
        for cell in cells
        {
            let cell_pos = cell.position;
            //let y = ((self.current_perception * self.current_perception - (i-cell_pos.x)) as f64).sqrt();
        }
    }

    /// Computes the current organism perception ability by adding the perception of each cell
    /// This function should be called when a cell is created or destroyed.
    pub fn update_perception(&mut self)
    {
        self.current_perception = 0;
        let cells = &self.cells;
        for cell in cells
        {
            self.current_perception += cell.perception;
        }
    }

    /// Returns the shape of an initially created organism
    pub fn get_initial_shape() -> Vec<Vec<bool>>
    {
        //   x = 0              x = 1
        //        y=0   y=1         y=0   y=1
        vec![vec![true, true], vec![true, true]]
    }

    /// Returns the cells composing the organism
    pub fn get_cells(&self) -> &Vec<Cell>
    {
        &self.cells
    }
}
