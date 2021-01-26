fn use_slices(slices: &mut [i32]){
  println!("{:?}",slices);
  println!("first = {},length = {}",slices[0],slices.len());
  slices[0] = 1000;
}

pub fn run(){
  let mut ary = [1,2,3,4,5];
  use_slices(&mut ary[1..4]);
  println!("{:?}",ary);
}