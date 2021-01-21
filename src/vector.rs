pub fn run(){
  let mut a = Vec::new();
  a.push(1);
  a.push(2);
  a.push(3);
  a.push(4);
  a.push(5);
  println!("{:?}",a);
  
  //iteration
  for x in &a{
    println!("{}",x);
  }

  match a.get(6) //options
  {
    Some(x) => println!("{}",x),
    None=>println!("No such element!")
  }
  
  // get pop elemet
  let last_num = a.pop();
  println!("last_num = {:?}",last_num);
  
  while let Some(x) = a.pop()
  {
    println!("{:?}",x);
  }
  
}