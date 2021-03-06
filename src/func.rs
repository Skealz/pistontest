//! Useful functions

use rusttype::{Point};

/// Check if the given position is available on the map
pub fn check_avail(map_usage : &Vec<Vec<bool>>, position : Point<i32>, space_needed : &Vec<Vec<bool>>) -> bool
{
    let mut is_okay = true;
    for index_to_addx in 0..space_needed.len()
    {
        for index_to_addy in 0..space_needed[index_to_addx].len()
        {
            if (position.x + index_to_addx as i32 >= map_usage.len() as i32 ||
            position.y + index_to_addy as i32 >= map_usage[position.x as usize + index_to_addx].len() as i32) ||
            (space_needed[index_to_addx][index_to_addy] == true &&
            map_usage[position.x as usize + index_to_addx][position.y as usize + index_to_addy] == true)
            {
                println!("nok");
                is_okay = false;
                break;
            }
        }
        if is_okay == false
        {
            break;
        }
    }
    is_okay
}

// Returns the available top left positions on the map where space is available to create a new cell.
// map_usage represents the current map usage.
// space_needed is the map containing the searched space.
/*pub fn get_avail_position(map_usage : Vec<Vec<bool>>, space_needed : &Vec<Vec<bool>>) -> Vec<Point<usize>>
{
    let mut avail_pos : Vec<Point<usize>> = Vec::new();
    for i in 0..WORLD_SIZE
    {
        for j in 0..WORLD_SIZE
        {
            let mut is_okay = true;
            for index_to_addx in 0..space_needed.len()
            {
                for index_to_addy in 0..space_needed[index_to_addx].len()
                {
                    if (i + index_to_addx >= map_usage.len() ||
                    j + index_to_addy >= map_usage[i + index_to_addx].len()) ||
                    (space_needed[index_to_addx][index_to_addy] == true &&
                    map_usage[i + index_to_addx][j + index_to_addy] == true)
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
            if is_okay
            {
                avail_pos.push(point(i, j));
            }
        }
    }
    avail_pos
}*/
