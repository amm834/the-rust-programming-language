pub fn run() {
  let a = 20;
  {
    let b = 30;
    // shadowing
    println!("inside ,a = {}", a);
    println!("inside ,b = {}", b);
  }
  println!("outside ,a = {}", a);
}