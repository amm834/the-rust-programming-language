pub fn run() {
  union();

}

union IntOrFloat {
  i: i32,
  f: f64
}

fn process_value(iof: IntOrFloat) {
  unsafe {
    match iof {
      IntOrFloat {
        i: 17
      } => println!("I am student."),
      IntOrFloat {
        f
      } => println!("{}", f)
    }
  }
}

fn union() {
  let mut iof = IntOrFloat {
    i: 123
  };
  iof.i = 17;
  process_value(iof);
  process_value(IntOrFloat {
    f: 17.0
  });

}