//! Gameboard

/// Size of the game board
const SIZE: usize = 9;

pub struct Gameboard
{
    //Stores the content of the cells composing the map. 0 means that a cell is empty.
    pub cells :[[u8; SIZE]; SIZE],
}

impl Gameboard
{
    pub fn new() -> Gameboard
    {
        Gameboard
        {
            cells: [[0; SIZE]; SIZE],
        }
    }
}
