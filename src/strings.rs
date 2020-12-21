pub fn run() {
    let hi = "Hi";

    let mut hello = String::from("hello people of the ");
    hello.push('W');
    hello.push_str("orld");

    println!("{}, {}", hi, hello);

    println!("\nGet string length");
    println!("Length: {}, {}", hi.len(), hello.len());
    println!("Capacity: {}", hello.capacity());

    println!("Contains 'world': {}", hello.contains("world"));
    println!("{}", hello.replace("world", "earth"));

    println!("\nSplit White Space");
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    println!("\nCreate a string with capacity");
    let mut my_string = String::with_capacity(20);
    my_string.push('a');
    my_string.push('b');
    my_string.push('c');
    my_string.push('a');
    my_string.push('b');
    my_string.push('c');
    my_string.push('a');
    my_string.push('b');
    my_string.push('c');
    
    println!("my_string capacity: {}", my_string.capacity());

    //
    my_string = String::from("   ");
    let your_string = my_string.trim();
    println!("my_string is empty: {}", my_string.is_empty());
    println!("d is empty: {}", your_string.is_empty());

    
    println!("\nString Contains");
    let contains_world = hello.contains("World");
    println!("Contains 'World'? {}", contains_world);

    println!("\nString Replace");
    let there = hello.replace("World", "Earth");
    println!("Replace 'World' with 'Earth': {}", there);

    println!("Assertion Testing");
    assert_eq!(3, my_string.len());
    assert_eq!(0, your_string.len());
}
