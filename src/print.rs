pub fn run() {
  println!("{} is from {}", "Brad", "Mass");
  
  println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");
  
  let name = "Brad";
  println!(
    "{name} likes to play {activity}.",
    name = name,
    activity = "baseball",
  );
  
  println!("{:b}\n{:x}\n{:o}", 10, 10, 10);

  println!("{:?}", (12, true, "hello"));

  println!("10 + 10 = {}", 10 + 10);
}