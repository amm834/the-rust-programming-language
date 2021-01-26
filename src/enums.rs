enum Color {
  RED,
  GREEN,
  BLUE,
  RGB(i32, i32, i32),
  CYMK{c: i32, y: i32, m: i32, k: i32}
}

pub fn run() {
  let c: Color = Color::CYMK{c:12,y:20,m:28,k:255};
  match c {
    Color::RED => println!("r"),
    Color::GREEN => println!("g"),
    Color::BLUE => println!("b"),
    Color::RGB(255, 255, 255) => println!("white"),
    Color::RGB(r, g, b) => println!("RGB({},{},{})", r, g, b),
    Color::CYMK{c:_,y:_,m:_,k:255} => println!("black"),
    _=>()
  }
}