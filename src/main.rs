extern crate rand;

use rand::Rng;
use std::cmp::min;
use std::cmp::max;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x_in: i32, y_in: i32, z_in: i32) -> Point {
        Point {
            x: x_in,
            y: y_in,
            z: z_in,
        }
    }
}

// floor and ceiling of terrain
const FLOR: i32 = 1;
const CEIL: i32 = 25;
// aggressiveness of changes from point to point such each point can be ±AGGR from neighboring points
const AGGR: i32 = 3;
// size of matrix
const HEIGHT: usize = 20;
const WIDTH: usize = 80;

fn main() {
    let mut m: Vec<Vec<Point>> = Vec::new();

    for i in 1..=HEIGHT {
        let mut r: Vec<Point> = Vec::new();
        m.push(r);
    }

    // current row and previous row of y values
    let mut curr_row: Vec<i32> = Vec::new();
    let mut prev_row: Vec<i32> = Vec::new();

    // point west of current point; initialized to 0
    let mut w = 0;

    let mut j: i32 = 0;

    for mut r in m {
        for i in 1..=WIDTH {

            // check for anything north of current point
            let n_opt = prev_row.get(i);
            match n_opt {
                // nothing north of current point
                None => {
                    if w > 0 {
                        // base on value to the west only
                        let rand_y = rand::thread_rng().gen_range(rand::thread_rng().gen_range(w - AGGR, w), rand::thread_rng().gen_range(w, w + AGGR));
                        r.push(Point::new(j, rand_y, i as i32));
                        // print!("{:03} ", rand_y);
                        w = rand_y;
                        curr_row.push(rand_y);
                    } else {
                        // base on no external value
                        let rand_y = rand::thread_rng().gen_range(FLOR, CEIL);
                        r.push(Point::new(j, rand_y, i as i32));
                        // print!("{:03} ", rand_y);
                        w = rand_y;
                        curr_row.push(rand_y);
                    }    
                },
                // something north of current point
                Some(n_val) => {   
                    if w > 0 {
                        // branch should never be true
                        // base on value to north and west
                        // let rand_y = rand::thread_rng().gen_range(rand::thread_rng().gen_range(FLOR, w), rand::thread_rng().gen_range(w, CEIL));

                        let n_lower = rand::thread_rng().gen_range(*n_val - AGGR, *n_val);
                        let n_upper = rand::thread_rng().gen_range(*n_val, *n_val + AGGR);

                        let rand_n = rand::thread_rng().gen_range(n_lower, n_upper);

                        let w_lower = rand::thread_rng().gen_range(w - AGGR, w);
                        let w_upper = rand::thread_rng().gen_range(w, w + AGGR);

                        let rand_w = rand::thread_rng().gen_range(w_lower, w_upper);

                        if rand_n == rand_w {
                            // if they're the same it panics
                            let rand_y = rand::thread_rng().gen_range(rand_n - AGGR, rand_n + AGGR);
                            r.push(Point::new(j, rand_y, i as i32));
                            // print!("{:03} ", rand_y);
                            w = rand_y;
                            curr_row.push(rand_y);
                        } else {
                            let rand_y = rand::thread_rng().gen_range(min(rand_n, rand_w), max(rand_n, rand_w));
                            r.push(Point::new(j, rand_y, i as i32));
                            // print!("{:03} ", rand_y);
                            w = rand_y;
                            curr_row.push(rand_y);
                        }                        
                    } else {
                        // Base on value to north only

                        let n_lower = rand::thread_rng().gen_range(*n_val - AGGR, *n_val);
                        let n_upper = rand::thread_rng().gen_range(*n_val, *n_val + AGGR);

                        let rand_y = rand::thread_rng().gen_range(n_lower, n_upper);
                        r.push(Point::new(j, rand_y, i as i32));
                        // print!("{:03} ", rand_y);
                        w = rand_y;
                        curr_row.push(rand_y);
                    }                         
                },
            }

                    
        }

        // clearing stored information at end of row
        for val in &curr_row {
            prev_row.push(*val);
        }
        let curr_row: Vec<i32> = Vec::new();
        w = 0;

        for p in &r {
            print!("{:03} ", p.y.to_string());
        }
        println!("");

        j += 1;
    }
}
