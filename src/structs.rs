// structs are like class
struct Point {
  x: u32,
  y: i32
}
struct Line {
  start: Point,
  end: Point
}
pub fn run() {
  // initialize and assign
  let p1 = Point {
    x: 1,
    y: 10
  };
  let p2 = Point {
    x: 5,
    y: 10
  };

  println!("p1.x = {}", p1.x);
  println!("p2.x = {}", p2.x);

  let line = Line {
    start: p1,
    end: p2
  };
  println!("{}", line.start.y);
}