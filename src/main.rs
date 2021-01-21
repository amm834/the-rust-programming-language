use std::mem;

fn main() {
  let a = 17;
  println!("a = {}", a);
  let mut b: usize= 0;
  println!("b = {}", b);
  b = 5;
  println!("Modificated b = {}", b);
  println!("Memory Size = {} bytes.",mem::size_of_val(&b));
  let z:isize = 123; //isize/usize
  let size_of_z = mem::size_of_val(&z);
  println!("z = {} ,takes {} bytes,on {}bit-os.",z,size_of_z,size_of_z * 8);
}