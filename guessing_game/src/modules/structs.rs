// to allow dead code 
#![allow(dead_code)]

#[derive(Debug)]
pub struct Person{
  first_name: String,
  last_name: String,
  age: u8,
}

pub struct Point{
  x: f32,
  y: f32,
}

pub fn use_struct(){
  let person = Person{
    first_name: String::from("John"),
    last_name: String::from("Doe"),
    age: 32
  };

  println!("use the struct variables {} {} {}", person.first_name, person.last_name, person.age);
}

pub struct Rectangle{
  width: f32,
  height: f32,
  top_left : f32
}

impl Rectangle {
  
  fn rect_area(&self, width:f32, height:f32) -> f32 {
    let area = width * height;
    area
  }

  fn square(&self, point: Point, another_point: f32) -> Rectangle{
    Rectangle {
      width: another_point,
      height: another_point,
      top_left: point.x
    }
  }
}

pub fn struct_activity(){
  let _point = Point{
    x: 32.0,
    y: 32.0,
  };
  let _rect = Rectangle{
    width: 12.0,
    height: 12.0,
    top_left: 22.0,
  };
  
 let area =  _rect.rect_area(_rect.width, _rect.height);
 let another_react = _rect.square(_point, 44.0);
 println!("print the area of rectangle {}", area);
 println!("print another rectangle {} {} {}", another_react.width, another_react.height, another_react.top_left);
}