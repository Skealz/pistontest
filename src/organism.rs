//! Organism living in the world

extern crate rand;

use Cell;
use constants::WORLD_SIZE;
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
    hunger : i16,

    /// Current hunger of the oranism (between 0 and hunger)
    current_hunger : i16,

    /// Current perception ability
    perception : i16,

    /// Current movement ability
    movement : i16,

    /// Points defining the area in which the organism can see
    perception_area : Vec<Point<i32>>,
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
            cells : cells,
            hunger : 10,
            current_hunger : 7,
            perception : 0,
            movement : 0,
            perception_area : Vec::new(),
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

    /// Function that computes the next position of the organism
    pub fn moving(&mut self, food_aim : &Point<i32>, closest_food_cell : &Point<i32>) -> bool
    {
        if food_aim.x != -1
        {
            let diff_x = food_aim.x - closest_food_cell.x;
            let diff_y = food_aim.y - closest_food_cell.y;
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
            }
        }
        false
    }

    /// Search for food in the perception area
    pub fn update_closest_food(&self, food: &Vec<Point<i32>>, food_aim : &mut Point<i32>, closest_food_cell : &mut Point<i32>)
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
        *food_aim = curr_food;
        *closest_food_cell = closest_cell_pos;
    }

    /// Updates the area in which the organism can see. Uses current_perception value
    /// added to the cells in all the directions
    fn update_perception_area(&mut self)
    {
        let cells = &self.cells;
        let mut circle_points : Vec<Point<i32>> = Vec::new();
        let r : f64 = self.perception as f64;
        let mut t = 0.0;
        while t <= 2.0*PI
        {
            for ray in (0..r as i32)
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
    fn update_perception_movement(&mut self)
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
