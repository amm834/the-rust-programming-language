use std::mem;
pub fn run(){
  let x = [1,2,3,4,5];
  println!("{:?} has {} bytes.",x,mem::size_of_val(&x));
  
  let y:[i32;5] = [1,2,3,4,5];
  println!("y[0] = {},it takes {} bytes.",y[0],mem::size_of_val(&y[0]));
  
  for i in 0..y.len(){
    println!("{}",y[i]);
  }
  //using array.iter()
  for i in y.iter(){
    println!("{}",i);
  }
  
  //multi-dimensional array
  let mta:[[i32;3];2] = 
  [
    [1,2,3],
    [4,5,6]
  ];
println!("{:#?}",mta);
for i in 0..mta.len(){
  for j in 0..mta[i].len(){
    println!("mta[{}][{}] = {}",i,j,mta[i][j]);
  }
}

}