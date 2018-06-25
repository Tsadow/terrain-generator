extern crate rand;

use rand::Rng;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn set_y(&mut self, yval: i32){
        self.y = yval;
    }
}

fn main() {
    let mut r1: Vec<Point> = Vec::new();
    let mut r2: Vec<Point> = Vec::new();
    let mut r3: Vec<Point> = Vec::new();

    let mut m: Vec<Vec<Point>> = Vec::new();

    m.push(r1);
    m.push(r2);
    m.push(r3);

    let mut j = 0;

    for mut r in m {
        for i in 1..=3 {
            r.push(Point {
                x: j,
                y: rand::thread_rng().gen_range(-10, 11),
                z: i,
            });         
        }

        for p in &r {
            print!("{} ", p.y.to_string());
        }
        println!("");

        j += 1;
    }
}
