//stack and heap
use std::mem;
struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 5.0, y: 10.0 }
}

pub fn run() {
    let p1 = origin();
    let p2 = Box::new(origin());
    let p3 = *p2;
    println!("Val of p1 = {}", mem::size_of_val(&p1));
    println!("{}", p3.x);
}
