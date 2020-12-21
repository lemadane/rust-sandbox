pub fn run() {

  println!("Shorthand if");

  let age: u8 = 30;
  let check_id: bool = false;
  
  let is_of_age = if age >= 21 { true } else { false };
  
  println!("Is Of Age: {}", is_of_age);

}