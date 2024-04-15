#![allow(dead_code)]

pub fn refs(v:Vec<i32>, v2:Vec<i32>) -> (Vec<i32>, Vec<i32>){
  for (count, value) in v.iter().enumerate(){
    println!("printing vec: index : {}, value: {}", count, value);
  }

  for (count, value) in v2.iter().enumerate(){
    println!("printing vec: index : {}, value: {}", count, value);
  }

  (v, v2)
}

pub fn borrow(v:&Vec<i32>)-> i32 {
  let mut sum= 0;
  for i in v{
    sum += i;
  }
  sum
}
