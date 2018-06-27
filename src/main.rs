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
    let mut m: Vec<Vec<Point>> = Vec::new();

    for i in 1..=10 {
        let r: Vec<Point> = Vec::new();
        m.push(r);
    }

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
