pub fn run() {

  println!("\nFunctions");

  greeting("Hello", "Jane");

  let sum = get_sum(5, 6);

  println!("\ngetsum(): {}", sum);

  println!("\nClosures");
  
  let n3 = 10;
  let add_num = |n1:i32, n2:i32| n1 + n2 - n3;

  println!("\nadd_num(): {}", add_num(10, 20));
}

fn greeting(greet: &str, name: &str) {
  println!("\n{} {}, nice to meet you!", greet, name);
}

fn get_sum(n1: i32, n2: i32) -> i32 {
  return n1 + n2;
}