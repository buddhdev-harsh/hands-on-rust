// // use std::io;


// #[allow(dead_code)]
// #[allow(unused_variables)]
// // mod modules;

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
use memchr::memrchr as find_char_reverse;
#[allow(unused_variables)]
#[allow(unused_assignments)]

fn parse(line: &str) {
  let mut filter_index_start: usize = 0;
  let mut filter_index_end: usize = line.len();

  let mut exception = false;
  if line.starts_with("@@") {
      filter_index_start += 2;
      exception = true;
      print!("1");
  }

  let maybe_options_index: Option<usize> = find_char_reverse(b'$', line.as_bytes());
  println!("{:?} 2", maybe_options_index);

  if let Some(options_index) = maybe_options_index {
      filter_index_end = options_index;

      // slicing here is safe; the first byte after '$' will be a character boundary
      let raw_options = &line[filter_index_end + 1..];
  println!("{:?} 3", raw_options);

      // options = Some(parse_filter_options(raw_options)?);
  }

  let left_anchor = if line[filter_index_start..].starts_with("||") {
      filter_index_start += 2;
      println!("|| 4");
  } else if line[filter_index_start..].starts_with('|') {
      filter_index_start += 1;
      println!("| 5");
  } else {
      println!("none 6");
  };
  println!("{:?} 7", left_anchor);

  let right_anchor = if filter_index_end > 0 && filter_index_end > filter_index_start && line[..filter_index_end].ends_with('|') {
      filter_index_end -= 1;
     print!("| 8");
  } else {
    println!("none 9");
  };
  println!("{:?} 10", right_anchor);

  let pattern = &line[filter_index_start..filter_index_end];

  println!("Done");
}

fn main(){
  let test = r"/^https?:\/\/[a-z]{8,15}\.top\/[a-z]{4,}\.json$/$xhr,3p,match-case";
  parse(test);

}
