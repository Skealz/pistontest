//! World

/// Size of the game board
const SIZE: usize = 9;

/// Stores game board information
pub struct World
{
    /// Stores the content of the cells composing the map. 0 means that a cell is empty.
    pub cells :[[u8; SIZE]; SIZE],
}

impl World
{
    /// Creates a new World
    pub fn new() -> World
    {
        World
        {
            cells: [[0; SIZE]; SIZE],
        }
    }
}
