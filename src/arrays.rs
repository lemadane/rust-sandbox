
use std::mem::size_of_val;

pub fn run() {
  let numbers: [i32; 5] = [1,2,3,4,5];
  println!("Numbers: {:?}", numbers);
  println!("Numbers: {}", numbers[0]);
  mutable_number[0] = 20;
  println!("Numbers: {:?}", mutable_number);
  println!("Numbers: {}", mutable_number[0]);
  println!("Array Length: {}", mutable_number.len());

  println!("Array occupies {} bytes", size_of_val(&mutable_number));

  println!("\nArray Slice");
  let slice = &mutable_number[0..3];
  println!("Slice: {:?}", slice);

}