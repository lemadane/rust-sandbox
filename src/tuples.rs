pub fn run() {
  println!("Tuples, max 12 elements");
  let p: (&str, &str, i8) = ("Brad", "Mass", 37);
  println!("{} , {}, {}", p.0, p.1, p.2); 
}