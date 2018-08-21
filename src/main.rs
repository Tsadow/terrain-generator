//! Tom Castle (@Tsadow)
//! 2018
//! 
//! Random terrain height map generation

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

fn main() {
    let height_map = gen_height_map(1, 13, 2, 5, 20, 50);
}

/// Generates height map based on supplied parameters
/// 
/// ### Parameters
/// ##### Generation Boundaries
/// `flor`: lowest possible value of terrain
/// `ceil`: highest possible value of terrain
/// ##### Aggressiveness: how far each point can possibly be from each other
/// `neg_aggr`: negative aggressiveness
/// `pos_aggr`: positive aggressiveness
/// ##### Output Boundaries
/// `height`: z-number of output
/// `width`: x-number of output
fn gen_height_map(flor: i32, ceil: i32, neg_aggr: i32, pos_aggr: i32, height: usize, width: usize) -> Vec<Vec<Point>> {
    let mut m: Vec<Vec<Point>> = Vec::new();

    for i in 1..=height {
        let mut r: Vec<Point> = Vec::new();
        m.push(r);
    }

    // current row and previous row of y values
    let mut curr_row: Vec<i32> = Vec::new();
    let mut prev_row: Vec<i32> = Vec::new();

    // point west of current point; initialized to 0
    let mut w = flor - 1;

    let mut j: i32 = 0;

    for mut r in &mut m {
        for i in 1..=width {

            // check for anything north of current point
            let n_opt = prev_row.get(i);
            match n_opt {
                // nothing north of current point
                None => {
                    if w > flor - 1 {
                        // base on value to the west only
                        let mut w_lower = rand::thread_rng().gen_range(w - neg_aggr, w);
                        let mut w_upper = rand::thread_rng().gen_range(w, w + pos_aggr);

                        if w_lower > ceil {
                            w_lower = ceil;
                        }
                        if w_lower < flor {
                            w_lower = flor;
                        }
                        if w_upper > ceil {
                            w_upper = ceil;
                        }
                        if w_upper < flor {
                            w_upper = flor;
                        }

                        if w_upper == w_lower {
                            if w_upper == ceil{
                                w_lower = ceil - pos_aggr;
                            }
                            if w_lower == flor {
                                w_upper = flor + neg_aggr;
                            }
                        }

                        let rand_y = rand::thread_rng().gen_range(w_lower, w_upper);
                        r.push(Point::new(j, rand_y, i as i32));
                        // print!("{:03} ", rand_y);
                        w = rand_y;
                        curr_row.push(rand_y);
                    } else {
                        // base on no external value
                        let rand_y = rand::thread_rng().gen_range(flor, ceil);
                        r.push(Point::new(j, rand_y, i as i32));
                        // print!("{:03} ", rand_y);
                        w = rand_y;
                        curr_row.push(rand_y);
                    }    
                },
                // something north of current point
                Some(n_val) => {   
                    if w > flor - 1 {
                        // base on value to north and west
                        // let rand_y = rand::thread_rng().gen_range(rand::thread_rng().gen_range(flor, w), rand::thread_rng().gen_range(w, ceil));

                        let n_lower = rand::thread_rng().gen_range(*n_val - neg_aggr, *n_val);
                        let n_upper = rand::thread_rng().gen_range(*n_val, *n_val + pos_aggr);

                        let mut rand_n = rand::thread_rng().gen_range(n_lower, n_upper);

                        let w_lower = rand::thread_rng().gen_range(w - neg_aggr, w);
                        let w_upper = rand::thread_rng().gen_range(w, w + pos_aggr);

                        let mut rand_w = rand::thread_rng().gen_range(w_lower, w_upper);

                        if rand_n > ceil {
                            rand_n = ceil;
                        }
                        if rand_n < flor {
                            rand_n = flor;
                        }
                        if rand_w > ceil {
                            rand_w = ceil;
                        }
                        if rand_w < flor {
                            rand_w = flor;
                        }

                        if rand_n == rand_w {
                            // if they're the same it panics
                            rand_n = max(rand_n - neg_aggr, flor);
                            rand_w = min(rand_n + neg_aggr, ceil);
                        } 

                        let rand_y = rand::thread_rng().gen_range(min(rand_n, rand_w), max(rand_n, rand_w));
                        r.push(Point::new(j, rand_y, i as i32));
                        // print!("{:03} ", rand_y);
                        w = rand_y;
                        curr_row.push(rand_y);                       
                    } else {
                        // Base on value to north only

                        let mut n_lower = rand::thread_rng().gen_range(*n_val - 2*neg_aggr, *n_val);
                        let mut n_upper = rand::thread_rng().gen_range(*n_val, *n_val + pos_aggr);

                        if n_lower > ceil {
                            n_lower = ceil;
                        }
                        if n_lower < flor {
                            n_lower = flor;
                        }
                        if n_upper > ceil {
                            n_upper = ceil;
                        }
                        if n_upper < flor {
                            n_upper = flor;
                        }

                        if n_upper == n_lower {
                            if n_upper == ceil{
                                n_lower = ceil - pos_aggr;
                            }
                            if n_lower == flor {
                                n_upper = flor + neg_aggr;
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
        let color_steps: i32 = (ceil - flor)/6;

        for p in r {
            let mut out: ColoredString = "".to_string().white();

            // color the output string based on y value so it's clearer how tall things are in terminal output
            if p.y < flor {
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