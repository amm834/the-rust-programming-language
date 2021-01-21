pub fn run() {
  //if_statement();
  //while_loop();
  for_loop();
}

fn if_statement() {
  let temp = 29;
  // expand style
  if temp < 10 {
    println!("Today is cool.");
  } else if temp > 30 {
    println!("Today is host")
  } else {
    println!("Today is OK.");
  }
  // inline style
  let today = if temp < 10 {
    "Cool"
  } else if temp > 30 {
    "Sunny"
  } else {
    "Ok"
  };
  println!("Today is {}", today);
}

fn while_loop() {
  let mut x = 2;
  while x < 1000 {
    x *= 2;
    if x == 64 {
      continue;
    }
    println!("x = {}", x);
  }
  println!("\n<------ Loop -------->\n");
  let mut y = 1;
  loop {
    y *= 2;
    if y == 1 << 10 {
      break;
    }
    println!("y = {}", y);
  }
}

fn for_loop() {
  for x in 1..11 //range between 1 to 11 equal to #[for(i=1;i<11;i++)]
  {
    if x == 5 {
      continue;
    }
    if x == 8 {
      break;
    }
    println!("{}", x);
  }

  // enumerate
  for (ind, val) in (40..51).enumerate() {
    println!("{} => {}", ind, val);
  }
}