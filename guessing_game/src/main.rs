// // use std::io;


#![allow(dead_code)]
// #[allow(unused_variables)]
mod modules;
use modules::enums::enums_activity;
// // use modules::if_else::check_number;
// // use modules::loops::{while_loop, for_loop, this_loop};
// // use modules::ref_bor::{borrow, refs};
// // use modules::strin::checks;
// fn main() {
//   // let mut input = String::new();
//   // println!("Enter the number : ");

//   // io::stdin().read_line(&mut input).unwrap();
//   // let num:i32 = input.trim().parse().unwrap();

//   // if_else condition 
//   // check_number(num);

//   // while loop 
//   // while_loop(num);
//   // for_loop(num);
//   // this_loop();

//   // reference and borrowing 
//   // let _v: Vec<i32> = vec![4, 5, 6, 5, 1, 10];
//   // let _v2: Vec<i32> = vec![10, 12, 14];

//   // let (_v, _v2) = refs(_v, _v2);
//   // let sum = borrow(&_v);

//   // println!("the summation of the values is : {}", sum);

//   // for i in _v{
//   //   println!("the val:{}", i);
//   // }

//   // the primitive type will work without implementing the borrowing concept example is given below.  
//   // let ch: char = 'l';
//   // use_ch(ch);
//   // print!("{}", ch);
  
//   // fibonacci(10);

//   // let mut x = 5;
//   // let y = &mut x;
//   // *y += 1;
//   // println!("{}", y);
//   // println!("{}", &x);

//   // string and &str diff
//   // let mut field_one= String::from("Username: ");
//   // let mut name= String::new();
//   // let mut _hell = ("yo bois").to_owned();
  
//   // println!("Enter the username: ");
//   // io::stdin().read_line(&mut name).unwrap();
//   // let mut name = name.trim().to_string();

//   // field_one.push_str(&name);
//   // println!("Field one: {}", field_one);

//   // let result = checks(&mut name);
//   // println!("Modified name: {}", result);
//   // fn get_things(number: i32) -> Option<bool>{
//   //   if number == 10 {
//   //     Some(true)
//   //   }else{
//   //     None
//   //   }
//   // }
//   // let number = 22;
//   // let things = get_things(number);
//   // let ans = match things{
//   //   Some(true) => print!("dadada"),
//   //   Some(false) => print!("nanana"),
//   //   None => print!("none"),
//   // };
//   // print!("{:?}", ans);
//   let mut number =Some(50);
//   while let Some(value) = number {
//     if value > 10 {
//       number = Some(value-1);
//     }else{
//       break;
//     }
//     println!("{:?}   ", value);
//   }
// }

// // fn use_ch(c:char){
// //   println!("{}", c);
// // }

// // fn fibonacci(val:i32){
// //   println!("the fibonacci sequence starts with: " );
// //   let (mut a, mut b)= (0, 1);
  
// //   for i in 1..=val {
// //     println!("the {} value {} in fibonacci sequence",i, a);
// //     let c = a + b;
// //     a = b;
// //     b = c;
// //   }
// // }
fn main(){
  enums_activity();
}
