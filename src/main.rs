extern crate rand;

use rand::Rng;

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
const FLOR: i32 = 0;
const CEIL: i32 = 25;
// aggressiveness of changes from point to point such each point can be ±AGGR from neighboring points
const AGGR: i32 = 3;

fn main() {
    let mut m: Vec<Vec<Point>> = Vec::new();

    for i in 1..=10 {
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
        let mut j0: usize = j as usize;
        for i in 1..=10 {
            let mut i0: usize = i as usize;

            // check for anything north of current point
            let n_opt = prev_row.get(i0);
            match n_opt {
                // nothing north of current point
                None => {
                    if w > 0 {
                        // base on value to the west only
                        let rand_y = rand::thread_rng().gen_range(rand::thread_rng().gen_range(w - AGGR, w), rand::thread_rng().gen_range(w, w+AGGR));
                        r.push(Point::new(j, rand_y, i));

                        w = rand_y;
                        curr_row.push(rand_y);
                    } else {
                        // base on no external value
                        let rand_y = rand::thread_rng().gen_range(FLOR, CEIL);
                        r.push(Point::new(j, rand_y, i));

                        w = rand_y;
                        curr_row.push(rand_y);
                    }    
                },
                // something north of current point
                Some(n_val) => {
                    // check for anything northwest of current point
                    let nw_opt = prev_row.get(i0 - 1);
                    match nw_opt {
                        // nothing northwest of current point
                        None => {
                            if w > 0 {
                                // branch should never be true
                                // base on value to north and west only
                                // let rand_y = rand::thread_rng().gen_range(rand::thread_rng().gen_range(FLOR, w), rand::thread_rng().gen_range(w, CEIL));
                                let rand_y = rand::thread_rng().gen_range(rand::thread_rng().gen_range(*n_val - AGGR, *n_val), rand::thread_rng().gen_range(*n_val, *n_val + AGGR));
                                r.push(Point::new(j, rand_y, i));

                                w = rand_y;
                                curr_row.push(rand_y);
                            } else {
                                // Base on value to north only
                                let rand_y = rand::thread_rng().gen_range(rand::thread_rng().gen_range(*n_val - AGGR, *n_val), rand::thread_rng().gen_range(*n_val, *n_val + AGGR));
                                r.push(Point::new(j, rand_y, i));

                                w = rand_y;
                                curr_row.push(rand_y);
                            } 
                        },
                        // something northwest of current point
                        Some(nw_val) => {
                            if w > 0 {
                                // base on all values to north, northwest, west
                                let rand_y = rand::thread_rng().gen_range(rand::thread_rng().gen_range(*n_val - AGGR, *n_val), rand::thread_rng().gen_range(*n_val, *n_val + AGGR));
                                r.push(Point::new(j, rand_y, i));

                                w = rand_y;
                                curr_row.push(rand_y);
                            } else {
                                // branch should never be true
                                // base on values to north and northwest only
                                let rand_y = rand::thread_rng().gen_range(rand::thread_rng().gen_range(*n_val - AGGR, *n_val), rand::thread_rng().gen_range(*n_val, *n_val + AGGR));
                                r.push(Point::new(j, rand_y, i));

                                w = rand_y;
                                curr_row.push(rand_y);
                            } 
                        }
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
