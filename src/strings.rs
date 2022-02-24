// Primitive str = Immutable fixed length string somewhere in memory
// String = growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");

    //Get Length
    println!("Length: {}", hello.len());

    hello.push('W'); // This push method is only for single characters
    hello.push_str("orld"); // This pushes string onto String

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty - returns bool
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World': {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion Testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}