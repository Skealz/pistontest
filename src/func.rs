//! Useful functions

use constants::SIZE;
use rusttype::{point,Point};


/// Returns the available top left positions on the map where space is available to create a new cell.
/// map_usage represents the current map usage.
/// space_needed is the map containing the searched space.
pub fn get_avail_position(map_usage : Vec<Vec<bool>>, space_needed : &Vec<Vec<bool>>) -> Vec<Point<usize>>
{
    let mut avail_pos : Vec<Point<usize>> = Vec::new();
    for i in 0..SIZE
    {
        for j in 0..SIZE
        {
            let mut is_okay = true;
            for index_to_addx in 0..space_needed.len()
            {
                for index_to_addy in 0..space_needed[index_to_addx].len()
                {
                    if map_usage[i + index_to_addx][j + index_to_addy] != space_needed[index_to_addx][index_to_addy]
                    {
                        is_okay = false;
                        break;
                    }
                }
                if is_okay == false
                {
                    break;
                }
            }
            avail_pos.push(point(i, j));
        }
    }
    avail_pos
}
