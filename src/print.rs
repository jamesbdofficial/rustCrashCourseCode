pub fn run() {
    //Print to console
    println!("hello world");

    //Basic Formatting
    //GIVES ERROR: println!(1);
    //"you might be missing a string literal to format with"
    // CORRECT: 
    println!("Number: {}", 1);

    //Positional Arguments
    println!("{} is from {} and is {} years old", "James", "St. Albans", 22);
    println!(
        "{0} is from {1} and {0} is {2} years old.",
        "James", "St. Albans", 22
    ); //result: James is from St. Albans and James is 22 years old.

    //Named Arguments
    println!(
        "{name} likes to play {activity}", 
        name="James",
        activity="football"
    );

    //Placeholder Traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello")); //Tuple datastructure here

    //Basic Math
    println!("10 + 10 = {}", 10+10);
}