//! Organism living in the world

extern crate rand;

use Cell;
use constants::SIZE;
use rusttype::{point,Point};
use func;
use organism::rand::Rng;

/// Living organism composed by cells.
pub struct Organism
{
    /// Cells that composes the organism
    cells : Vec<Cell>,

    /// Size of the hunder gauge
    hunger : i16,

    /// Current hunger of the oranism (between 0 and hunger)
    current_hunger : i16,
}

impl Organism
{
    /// Creates a new organism
    /// map_usage represents the current map usage.
    pub fn new(map_usage : Vec<Vec<bool>>) -> Organism
    {
        let mut cells : Vec<Cell> = Vec::new();

        let initial_shape = Organism::get_initial_shape();

        let avail_pos = func::get_avail_position(map_usage, &initial_shape);
        let chosed_pos_index = rand::thread_rng().gen_range(0, avail_pos.len());

        for i in 0..initial_shape.len()
        {
            for j in 0..initial_shape[i].len()
            {
                if initial_shape[i][j]
                {
                    cells.push(Cell::new_random(point(avail_pos[chosed_pos_index].x + i, avail_pos[chosed_pos_index].y + j)));
                }
            }
        }

        println!("{}", cells.len());



        Organism
        {
            cells : cells,
            hunger : 10,
            current_hunger : 7,
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
