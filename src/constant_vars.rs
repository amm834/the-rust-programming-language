const AGE: i64 = 17; //no fixed adress
static mut JOB: &str = "Student";
pub fn run() {
    println!("Value of AGE = {}", AGE);
    unsafe // to mutate `static` variable
  {
  JOB = "Programmer";
  println!("Value of JOB = {}",JOB);
  }
}
