// Primitive str = Immutable fixed-length string somewhere in memory
// String = growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let hello = "Hello";
    let mut mutHello = String::from("Hello asdf asdf");

    // Get Length
    println!("Length: {}", hello.len());
    // Push char
    mutHello.push('W');
    // Push string
    mutHello.push_str("orld!");
    // Capacity in bytes
    println!("Capacity: {}", mutHello.capacity());

    println!("Is Empty: {}", mutHello.is_empty());

    println!("Contains 'World' {}", mutHello.contains("World"));
    // Replace
    println!("Replace: {}", mutHello.replace("World","There"));
    // Loop through string by whitespace
    for word in mutHello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);

    // Assertion testing
    assert_eq!(0, s.len());

    println!("{}", mutHello);
}