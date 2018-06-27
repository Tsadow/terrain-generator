extern crate rand;

use rand::Rng;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

// impl Point {
//     fn set_y(&mut self, yval: i32){
//         self.y = yval;
//     }
// }

fn main() {
    let r1: Vec<Point> = Vec::new();
    let r2: Vec<Point> = Vec::new();
    let r3: Vec<Point> = Vec::new();
    let r4: Vec<Point> = Vec::new();
    let r5: Vec<Point> = Vec::new();

    let mut m: Vec<Vec<Point>> = Vec::new();

    m.push(r1);
    m.push(r2);
    m.push(r3);
    m.push(r4);
    m.push(r5);

    let mut j = 0;

    for mut r in m {
        for i in 1..=10 {
            r.push(Point {
                x: j,
                y: rand::thread_rng().gen_range(0, 20),
                z: i,
            });         
        }

        for p in &r {
            print!("{:03} ", p.y.to_string());
        }
        println!("");

        j += 1;
    }
}
