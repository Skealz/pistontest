//! Organism living in the world

extern crate rand;

use Cell;
use constants::*;
use rusttype::{Point, point};
use func;
use rand::Rng;
use std::f64::consts::PI;
use std::f64::INFINITY;
use World;

/// Living organism composed by cells.
pub struct Organism
{
    /// Cells that composes the organism
    cells : Vec<Cell>,

    /// Size of the hunder gauge
    max_hunger : i16,

    /// Current hunger of the oranism (between 0 and max_hunger)
    hunger : i16,

    /// Current life
    life : i16,

    /// Current perception ability
    perception : i16,

    /// Current movement ability
    movement : i16,

    /// Points defining the area in which the organism can see
    perception_area : Vec<Point<i32>>,

    /// Temporary direction when no food is found
    temp_direction : Point<i32>

}

impl Organism
{
    /// Creates a new organism
    /// map_usage represents the current map usage.
    pub fn new(map_usage : Vec<Vec<bool>>) -> Organism
    {
        let mut cells : Vec<Cell> = Vec::new();

        let initial_shape = Organism::get_initial_shape();

        let mut chosed_pos = point(rand::thread_rng().gen_range(0, WORLD_SIZE) as i32, rand::thread_rng().gen_range(0, WORLD_SIZE) as i32);
        while !func::check_avail(&map_usage, chosed_pos, &initial_shape)
        {
            chosed_pos = point(rand::thread_rng().gen_range(0, WORLD_SIZE) as i32, rand::thread_rng().gen_range(0, WORLD_SIZE) as i32);
        }

        println!("{:?}", chosed_pos);

        for i in 0..initial_shape.len()
        {
            for j in 0..initial_shape[i].len()
            {
                if initial_shape[i][j]
                {
                    cells.push(Cell::new_random(point(chosed_pos.x + i as i32, chosed_pos.y + j as i32)));
                }
            }
        }

        let mut org = Organism
        {
            max_hunger : (cells.len() as f32 * CELL_FOOD).round() as i16,
            hunger : (cells.len() as f32 * CELL_FOOD).round() as i16,
            cells : cells,
            perception : 0,
            movement : 0,
            perception_area : Vec::new(),
            temp_direction : point(-1,-1),
            life : 0,
        };
        org.update_perception_movement();
        org.update_perception_area();
        org
    }

    /// Updates the organism
    pub fn update(&mut self, food: &Vec<Point<i32>>)
    {
        //println!("OrgUpdate");

        /*if self.food_aim.x == -1
        {
            self.update_closest_food(food);
            //println!("Closest food : {:?}", self.food_aim);
        }
        s.moving();*/
    }

    /// Apply effects when organism is eating food
    pub fn eating(&mut self)
    {
        self.hunger += FOOD_VALUE as i16;
        let avail_pos = self.find_avail_cell_pos();
        let idx = rand::thread_rng().gen_range(0, avail_pos.len()) as usize;
        // Create new cell her
        self.cells.push(Cell::new_random(*avail_pos.get(idx).unwrap()));
    }

    /// Find available positions for a new cell
    pub fn find_avail_cell_pos(&self) -> Vec<Point<i32>>
    {
        let mut avail_pos : Vec<Point<i32>> = Vec::new();
        for cell in &self.cells
        {
            let mut pos : Vec<Point<i32>> =  Vec::new();
            pos.push(point(cell.position.x + 1, cell.position.y));
            pos.push(point(cell.position.x, cell.position.y + 1));
            pos.push(point(cell.position.x - 1, cell.position.y));
            pos.push(point(cell.position.x, cell.position.y - 1));

            for p in pos
            {
                let mut p_ok = true;
                for cel in &self.cells
                {
                    if cel.position == p
                    {
                        p_ok = false;
                        break;
                    }
                }

                if p_ok
                {
                    avail_pos.push(p);
                }
            }
        }
        //println!("Cell pos {:?}", self.cells.get(0).unwrap().position);
        //println!("{:?}", avail_pos);
        avail_pos
    }

    /// Function that computes the next position of the organism
    /// Returns true if the targeted point is reached.
    /// The targeted point is food_aim, or if food_aim contains (-1,-1), the target is
    /// taken in the organism attribute temp_direction.
    pub fn moving(&mut self, food_aim : &Point<i32>, closest_food_cell : &Point<i32>) -> bool
    {
        let mut goal = *food_aim;
        if goal.x == -1
        {
            goal = self.temp_direction;
            //println!("Moving without food {:?}", goal);
        }

        let diff_x = goal.x - closest_food_cell.x;
        let diff_y = goal.y - closest_food_cell.y;
        let dist = i32::abs(diff_x) + i32::abs(diff_y);
        // If the distance is inferior than the movement ability of the organism,
        // directly set the closest cell on the food
        if dist <= self.movement as i32
        {
            for cell in &mut self.cells
            {
                cell.position.x += diff_x;
                cell.position.y += diff_y;
            }
            //Moving perception area
            for perc_pos in &mut self.perception_area
            {
                perc_pos.x += diff_x;
                perc_pos.y += diff_y;
            }
            //println!("REACHED AIMED POINT");
            return true
        }
        else
        {
            let fact_x = diff_x as f64 / dist as f64;
            let fact_y = diff_y as f64 / dist as f64;
            // one must be rounded bottom and the other up.. Care about numbers .5.. that will
            // be rounded up both..
            let mut mov_x : f64 = self.movement as f64 * fact_x;
            let mut mov_y : f64 = self.movement as f64 * fact_y;
            if mov_x - mov_x.trunc() == mov_y - mov_y.trunc()
            {
                mov_x = mov_x.trunc();
                mov_y = mov_y.ceil();
            }
            else
            {
                mov_x = mov_x.round();
                mov_y = mov_y.round();
            }
            //println!("mov_x : {:?}, mov_y : {:?}", mov_x, mov_y);
            for cell in &mut self.cells
            {
                cell.position.x += mov_x as i32;
                cell.position.y += mov_y as i32;
            }
            for perc_pos in &mut self.perception_area
            {
                perc_pos.x += mov_x as i32;
                perc_pos.y += mov_y as i32;
            }
        }
        false
    }

    /// Search for food in the perception area
    /// If food is found, the function returns true and the aimed position is set in food_aim and
    /// the position of the closest organism cell in closest_food_cell
    /// If no food is found a random direction is selected : the aimed position is set in food_aim and the
    /// closest cell positiion to this position in closest_food_cell. When no food is found, false is returned
    pub fn update_closest_food(&self, food: &Vec<Point<i32>>, food_aim : &mut Point<i32>, closest_food_cell : &mut Point<i32>) -> bool
    {
        //let food = world.get_food_pos();
        let mut curr_food = point(-1,-1);
        let mut closest_cell_pos = point(-1, -1);
        let mut has_food = false;
        let mut min_food_dist = INFINITY;
        // Iterate through all the perception points
        for pnt in &self.perception_area
        {
            if food.contains(&pnt)
            {
                // If there is food at this location, iterate through each cell and store the
                // distance between the closest cell and the food
                if has_food
                {
                    let mut min_dist = INFINITY;
                    let mut cell_pos = point(-1, -1);
                    for cell in &self.cells
                    {
                        let dist = f64::sqrt(((cell.position.x - pnt.x).pow(2) + (cell.position.y - pnt.y).pow(2)) as f64);
                        if dist < min_dist
                        {
                            min_dist = dist;
                            cell_pos = cell.position;
                        }
                    }
                    if min_dist < min_food_dist
                    {
                        min_food_dist = min_dist;
                        closest_cell_pos = cell_pos;
                        curr_food = *pnt;
                    }
                }
                else //If no food is set
                {
                    curr_food = *pnt;

                    let mut min_dist = INFINITY;
                    let mut cell_pos = point(-1, -1);
                    for cell in &self.cells
                    {
                        let dist = f64::sqrt(((cell.position.x - pnt.x).pow(2) + (cell.position.y - pnt.y).pow(2)) as f64);
                        if dist < min_dist
                        {
                            cell_pos = cell.position;
                            min_dist = dist;
                        }
                    }
                    closest_cell_pos = cell_pos;
                    min_food_dist = min_dist;
                    has_food = true;
                }
            }
        }
        // If no food has been found in perception range, the organism goes in a random direction
        if !has_food
        {
            // If no current temp random direction is set, we compute one.
            // Otherwise we take the one already computed
            if self.temp_direction.x == -1
            {
                let mut circle_points : Vec<Point<i32>> = Vec::new();
                let mut t = 0.0;
                let r : f64 = self.perception as f64;
                while t <= 2.0*PI
                {
                    circle_points.push(point((r as f64 * f64::cos(t)).round() as i32, (r as f64 * f64::sin(t)).round() as i32));
                    t += 0.05;
                }
                let idx = rand::thread_rng().gen_range(0, circle_points.len()) as usize;
                *food_aim = point(self.cells[0].position.x + circle_points[idx].x, self.cells[0].position.y + circle_points[idx].y);
                //println!("NEW diiir {:?}", food_aim);
            }
            else
            {
                *food_aim = self.temp_direction;
            }
            let mut min_dist = INFINITY;
            for cell in &self.cells
            {
                let dist = f64::sqrt(((cell.position.x - food_aim.x).pow(2) + (cell.position.y - food_aim.y).pow(2)) as f64);
                if dist < min_dist
                {
                    min_dist = dist;
                    *closest_food_cell = cell.position;
                }
            }
            return false;
        }

        *food_aim = curr_food;
        *closest_food_cell = closest_cell_pos;
        return true
    }

    /// Updates the area in which the organism can see. Uses current_perception value
    /// added to the cells in all the directions
    pub fn update_perception_area(&mut self)
    {
        let cells = &self.cells;
        let mut circle_points : Vec<Point<i32>> = Vec::new();
        let r : f64 = self.perception as f64;
        let mut t = 0.0;
        while t <= 2.0*PI
        {
            for ray in 0..r as i32
            {
                circle_points.push(point((ray as f64 * f64::cos(t)).round() as i32, (ray as f64 * f64::sin(t)).round() as i32));
            }
            t += 0.05;
        }
        for cell in cells
        {
            let cell_pos = cell.position;
            for circle_point in &circle_points
            {
                self.perception_area.push(point(cell_pos.x as i32 + circle_point.x, cell_pos.y as i32 + circle_point.y));
            }
        }
    }

    /// Computes the current organism perception and movement ability by adding the perception and movement of each cell
    /// This function should be called when a cell is created or destroyed.
    pub fn update_perception_movement(&mut self)
    {
        self.perception = 0;
        self.movement = 0;
        let cells = &self.cells;
        for cell in cells
        {
            self.perception += cell.perception;
            self.movement += cell.movement;
        }
    }

    /// Update life gauge
    pub fn update_life(&mut self)
    {
        let min_neigh = INFINITY;
        let mut life = 0;
        for cell in &self.cells
        {
            life += cell.curr_life;
        }
        self.life = life;
    }

    /// This function must be called when the organism is starving (hunger == 0)
    /// It will remove life from the latest added cell
    pub fn starving(&mut self)
    {
        //let idx = rand::thread_rng().gen_range(0, self.cells.len()) as usize; to select random cell
        //self.cells[idx].curr_life -= 1;
        let mut cellOpt = self.cells.last();
        if cellOpt.is_some()
        {
            let mut cell = cellOpt.as_mut().unwrap();//.curr_life -= 1;
            cell.curr_life -= 1;
        }
        else
        {
            println!("[STARVING] No cell from which to remove life");
        }
    }

    /// Update hunger gauge
    pub fn update_hunger(&mut self)
    {
        if self.hunger >= 0
        {
            self.hunger = (self.hunger as f32 - (self.cells.len() as f32 * CELL_CONSUMP)).round() as i16;
            println!("HUNGER : {:?}", self.hunger);
        }
    }

    ///Return current hunger
    pub fn get_hunger(&mut self) -> i16
    {
        return self.hunger;
    }

    /// Remove cells that have curr_life at zero
    pub fn remove_dead_cells(&mut self)
    {
        let mut rm_idx : Vec<usize> = Vec::new();
        for i in 0..self.cells.len()
        {
            if self.cells[i].curr_life <= 0
            {
                rm_idx.push(i);
            }
        }

        for idx in rm_idx
        {
            self.cells.remove(idx);
        }
    }

    /// Set temporary direction
    pub fn set_temp_dir(&mut self, direction: &Point<i32>)
    {
        self.temp_direction = *direction;
    }

    /// Returns the points composing the circle of the perception area
    pub fn get_perception_area(&self) -> &Vec<Point<i32>>
    {
        &self.perception_area
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

impl PartialEq for Organism {
    fn eq(&self, other: &Organism) -> bool {
        self.cells[0].position == other.cells[0].position
    }
}
