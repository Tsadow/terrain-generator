extern crate rand;
extern crate colored;

use rand::Rng;
use std::cmp::min;
use std::cmp::max;
use colored::*;

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
// it's recommended that CEIL - FLOR is evenly divisible by 6 for color visuialization to work best
const FLOR: i32 = 1;
const CEIL: i32 = 13;
// aggressiveness of changes from point to point such each point can be ±AGGR from neighboring points
const NEG_AGGR: i32 = 2;
const POS_AGGR: i32 = 5;
// size of matrix
const HEIGHT: usize = 20;
const WIDTH: usize = 70;

fn main() {
    let height_map = gen_height_map();
}

fn gen_height_map() -> Vec<Vec<Point>> {
    let mut m: Vec<Vec<Point>> = Vec::new();

    for i in 1..=HEIGHT {
        let mut r: Vec<Point> = Vec::new();
        m.push(r);
    }

    // current row and previous row of y values
    let mut curr_row: Vec<i32> = Vec::new();
    let mut prev_row: Vec<i32> = Vec::new();

    // point west of current point; initialized to 0
    let mut w = FLOR - 1;

    let mut j: i32 = 0;

    for mut r in &mut m {
        for i in 1..=WIDTH {

            // check for anything north of current point
            let n_opt = prev_row.get(i);
            match n_opt {
                // nothing north of current point
                None => {
                    if w > FLOR - 1 {
                        // base on value to the west only
                        let mut w_lower = rand::thread_rng().gen_range(w - NEG_AGGR, w);
                        let mut w_upper = rand::thread_rng().gen_range(w, w + POS_AGGR);

                        if w_lower > CEIL {
                            w_lower = CEIL;
                        }
                        if w_lower < FLOR {
                            w_lower = FLOR;
                        }
                        if w_upper > CEIL {
                            w_upper = CEIL;
                        }
                        if w_upper < FLOR {
                            w_upper = FLOR;
                        }

                        if w_upper == w_lower {
                            if w_upper == CEIL{
                                w_lower = CEIL - POS_AGGR;
                            }
                            if w_lower == FLOR {
                                w_upper = FLOR + NEG_AGGR;
                            }
                        }

                        let rand_y = rand::thread_rng().gen_range(w_lower, w_upper);
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
                    if w > FLOR - 1 {
                        // base on value to north and west
                        // let rand_y = rand::thread_rng().gen_range(rand::thread_rng().gen_range(FLOR, w), rand::thread_rng().gen_range(w, CEIL));

                        let n_lower = rand::thread_rng().gen_range(*n_val - NEG_AGGR, *n_val);
                        let n_upper = rand::thread_rng().gen_range(*n_val, *n_val + POS_AGGR);

                        let mut rand_n = rand::thread_rng().gen_range(n_lower, n_upper);

                        let w_lower = rand::thread_rng().gen_range(w - NEG_AGGR, w);
                        let w_upper = rand::thread_rng().gen_range(w, w + POS_AGGR);

                        let mut rand_w = rand::thread_rng().gen_range(w_lower, w_upper);

                        if rand_n > CEIL {
                            rand_n = CEIL;
                        }
                        if rand_n < FLOR {
                            rand_n = FLOR;
                        }
                        if rand_w > CEIL {
                            rand_w = CEIL;
                        }
                        if rand_w < FLOR {
                            rand_w = FLOR;
                        }

                        if rand_n == rand_w {
                            // if they're the same it panics
                            rand_n = max(rand_n - NEG_AGGR, FLOR);
                            rand_w = min(rand_n + NEG_AGGR, CEIL);
                        } 

                        let rand_y = rand::thread_rng().gen_range(min(rand_n, rand_w), max(rand_n, rand_w));
                        r.push(Point::new(j, rand_y, i as i32));
                        // print!("{:03} ", rand_y);
                        w = rand_y;
                        curr_row.push(rand_y);                       
                    } else {
                        // Base on value to north only

                        let mut n_lower = rand::thread_rng().gen_range(*n_val - NEG_AGGR, *n_val);
                        let mut n_upper = rand::thread_rng().gen_range(*n_val, *n_val + POS_AGGR);

                        if n_lower > CEIL {
                            n_lower = CEIL;
                        }
                        if n_lower < FLOR {
                            n_lower = FLOR;
                        }
                        if n_upper > CEIL {
                            n_upper = CEIL;
                        }
                        if n_upper < FLOR {
                            n_upper = FLOR;
                        }

                        if n_upper == n_lower {
                            if n_upper == CEIL{
                                n_lower = CEIL - POS_AGGR;
                            }
                            if n_lower == FLOR {
                                n_upper = FLOR + NEG_AGGR;
                            }
                        }

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

        // get number of numbers in each color
        let color_steps: i32 = (CEIL - FLOR)/6;

        for p in r {
            let mut out: ColoredString = "".to_string().white();

            // color the output string based on y value so it's clearer how tall things are in terminal output
            if p.y < FLOR {
                out = p.y.to_string().black();
            } else if p.y <= color_steps {
                out = p.y.to_string().magenta();
            } else if p.y <= color_steps*2 {
                out = p.y.to_string().blue();
            } else if p.y <= color_steps*3 {
                out = p.y.to_string().cyan();
            } else if p.y <= color_steps*4 {
                out = p.y.to_string().green();
            } else if p.y <= color_steps*5 {
                out = p.y.to_string().yellow();
            } else if p.y <= color_steps*6 {
                out = p.y.to_string().red();
            } else {
                out = p.y.to_string().white();
            }

            print!("{:03} ", out);
        }
        println!("");

        j += 1;
    }

    m
}