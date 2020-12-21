
use std::mem::size_of_val;

pub fn run() {
  println!("\nVectors");
  let mut numbers: Vec<i32> = vec![1,2,3,4,5];
  println!("Numbers: {:?}", numbers);
  println!("Numbers: {}", numbers[0]);

  numbers.push(6);
  numbers.push(7);
  
  println!("Numbers: {:?}", numbers);
  numbers.pop();
  println!("Numbers: {:?}", numbers);

  println!("Array Length: {}", numbers.len());

  println!("Array occupies {} bytes", size_of_val(&numbers));

  println!("\nVector Slice");
  let slice = &numbers[0..3];
  println!("Slice: {:?}", slice);

  println!("Loop Vector");

  for x in numbers.iter_mut() {
    *x = *x * 2;
    println!("Number {}", x);
  }
  println!("Numbers: {:?}", numbers);
}