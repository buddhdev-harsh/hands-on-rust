
fn main(){
  let mut name = String::from("harsh");

  let name_copy = &name;

  println!("name and copied name {},  {}", name, name_copy);

  name = "not a problem".to_string();
  let name_copy = "jogger";

  println!("name after putting mut {}, {}", name_copy, name);
  let mut var: i32 = 38;
  let var_to_add : i32 = 10;
  var = take_ownership(&mut var, var_to_add);
  println!("here is the variable :{}", var);

}

fn take_ownership(var_name:&mut i32, val_to_add: i32) -> i32 {
  *var_name += val_to_add;
  *var_name
}