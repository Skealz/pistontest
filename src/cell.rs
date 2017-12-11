//! Cells behavior

extern crate rand;
use rusttype::Point;
use cell::rand::Rng;

pub enum CellType
{
    Attack,
    Defense,
    Movement,
    Life,
    Perception,
}

/// Cell struct
pub struct Cell
{
    /// Cell max life
    pub life : i16,

    /// Cell current life
    pub curr_life : i16,

    /// Ability of the cell to move
    pub movement : i16,

    /// Ability of the cell to attack another cell
    pub attack : i16,

    /// Ability of the cell to protect the organism
    pub defense : i16,

    /// Ability of the cell de perceive its surrounding
    pub perception : i16,

    /// Cell position on the grid
    pub position : Point<usize>,
}

impl Cell
{
    /// Creates a new cell with a predefined type at the specified position
    pub fn new(cell_type : CellType, position : Point<usize>) -> Cell
    {
        println!("Creating cell");
        Cell
        {
            position,
            life : match cell_type { CellType::Life => 10, _ => 1 },
            curr_life : match cell_type { CellType::Life => 10, _ => 1 },
            movement : match cell_type { CellType::Movement => 3, _ => 1 },
            attack : match cell_type { CellType::Attack => 10, _ => 1 },
            defense : match cell_type { CellType::Defense => 10, _ => 1 },
            perception : match cell_type { CellType::Perception => 5, _ => 1 },
        }
    }

    /// Creates a new cell with a random type at the specified position
    pub fn new_random(position : Point<usize>) -> Cell
    {
        let type_index = rand::thread_rng().gen_range(0, 4);

        if type_index == 0
        {
            Cell::new(CellType::Attack, position)
        }
        else if type_index == 1
        {
            Cell::new(CellType::Defense, position)
        }
        else if type_index == 2
        {
            Cell::new(CellType::Movement, position)
        }
        else if type_index == 3
        {
            Cell::new(CellType::Life, position)
        }
        else
        {
            Cell::new(CellType::Perception, position)
        }
    }
}
