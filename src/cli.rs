use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  println!("Args: {:?}", args);
  let command = args[1].clone();
  let name = args[2].clone();
  let status = "100%";
  
  if command == "hello" {
    println!("{} {}, how are you?", command, name);
  } else if command == "status" {
    println!("Status is {}", status);
  } else {
    println!("That is not a valid command");
  }
}