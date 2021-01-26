fn add_and_mutiplication(x: i32, y: i32)->(i32, i32) {
  (x+y, x*y)
}

pub fn run() {
  let a = 10;
  let b = 5;
  let add_and_multi = add_and_mutiplication(a, b);
  println!("{0}+{1} = {2},{0}*{1} = {3}", a, b, add_and_multi.0, add_and_multi.1);
  // destructring
  let combine = add_and_multi;
  println!("{:#?}", combine);
  let (c, d) = combine;
  println!("c ={},d = {}", c, d);
  let meaning = (combine, (1, 2));
  println!("{:?}", meaning);
  println!("last num = {}", meaning.1.1);
  let ((e, f), (g, h)) = meaning;
  println!("h = {}", h);
}