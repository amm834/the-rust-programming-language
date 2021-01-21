pub fn run(){
  let a = 5;
  let b = 3;
  let addition = a + b; // +-*/%
  println!("{} + {} = {}",a,b,addition);
  
  //logical operators
  let x =  1|2 ; // | OR , ! NOR ,& AND 
  println!("1|2 = {}",x);
  
  // bitwise
  let shift = 1 << 10; // << left shift , >> right shift
  println!("Left shift {}",shift);
  
  // PI
  println!("Value of PI = {}",std::f64::consts::PI); // std::f32::consts::PI for 32 bit
  println!("3^3 = {}",i32::pow(b,3));
  println!("3^PI = {}",f64::powf(b.into(),std::f64::consts::PI));
}