#![allow(dead_code)]
const THRESHOLD: i32 = 10;

fn check_thresold(val: i32) -> bool{
  val > THRESHOLD
}
struct Point {
  x: i32,
  y: i32,
}

enum MouseFunctions{
  Click(Point),
  Position(Point),
  RightClick
}
type Operations= MouseFunctions;
fn inspect(op: Operations) {
  match op {
    Operations::Click(point)=> println!("this is it x: {}, y: {}",point.x, point.y ),
    Operations::Position(current) => println!("This is where the pointer is right now x:{}, y:{}", current.x, current.y),
    Operations::RightClick => println!("Mouse right clicked!"),
  }
}


pub fn enums_activity(){
  let click_pointer = Point{
    x: 12,
    y: 12,
  };
  let change_position = Point {
    x: 22,
    y: 22,
  };

  let click = Operations::Click(click_pointer);
  let change_positions = Operations::Position(change_position);

  inspect(click);
  inspect(change_positions);
  let number: i32= 22;
  println!("{} is greater than THRESHOLD?: {}", number,check_thresold(number));
}