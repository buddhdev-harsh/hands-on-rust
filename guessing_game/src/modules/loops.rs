pub fn while_loop(mut count: i32){
  
  while count != 40 {
    println!("the count number is: {}", count);
    count += 1;
  }
}

pub fn for_loop(count: i32 ){

  for i in 1..count{
    println!("this is number :{}", i);

    if i == 40 {
      break;
    }
  }
}

pub fn this_loop(){
  // for i in (1..=30).step_by(2){
  //   println!("the number is : {}", i);
  // }

  for (ind, val) in (5..=10).enumerate(){
    println!("the index {ind} and the number {val}" );
  }
}

