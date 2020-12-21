pub fn run(name: &str) {
  let mut age = 37;
  println!("My name is {} from {}", name, age);
  age = 38;
  println!("My name is {} from {}", name, age);

  const ID:i32 = 123;
  const PI:f32 = 3.9;

  println!("ID = {}", ID);
  println!("PI = {}", PI);

  let (my_name, my_age, my_phone) = ("Brad", 37, "1234567890");
  println!("{} {} {}", my_name, my_age, my_phone);
}