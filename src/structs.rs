struct Color {
  red: u8,
  green: u8,
  blue: u8
}

struct Colour(u8, u8, u8);

struct Name {
  first: String,
  last: String,
}

impl Name {
  fn new(first: &str, last: &str) -> Name {
    let person = Name {
      first: first.to_string(),
      last: last.to_string(),
    };
    person
  }

  fn full(&self) -> String {
    format!("{} {}", self.first, self.last)
  }

  fn set_last(&mut self, last: &str) -> String {
    self.last = last.to_string();
    self.full()
  }

  fn to_tuple(self) -> (String, String) {
    (self.first, self.last)
  }
}

pub fn run() {
  let mut color = Color {
    red: 255,
    green: 0,
    blue: 0,
  };
  color.red = 200;

  println!("Color: {:?}", (color.red, color.green, color.blue));

  let mut colour = Colour(255, 0, 0);

  colour.0 = 200;

  println!("Colour: {:?}", (colour.0, colour.1, colour.2));

  let mut name = Name::new("Joan", "Doe");

  // println!("Person: {} {}", name.first, name.last);

  println!("Person: {}", name.full());

  println!("Person: {}", name.set_last("Smith"));
  
  println!("Person: {:?}", name.to_tuple());
}