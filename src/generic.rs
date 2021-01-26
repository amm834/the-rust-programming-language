
struct Point<T>{
  x:T,
  y:T
}
struct Line<T>{
  start:Point<T>,
  end:Point<T>
}
pub fn run(){
  let a = Point{x:6,y:5};
  let b = Point{x:10,y:10};
  let line = Line{start:a,end:b};
}