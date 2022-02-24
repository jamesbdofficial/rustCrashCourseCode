// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "James";
    
    let age = 22;
    // 'age = 23;' will throw warning: "value assigned to 'age' is never read"
    println!("{}", age);
    // Like using const in c#
    // By making variable mutable, it's value can change
    let mut mutable_age = 22;
    mutable_age = 23;

    println!("My name is {} and I am {}", name, mutable_age);

    //define constant (MUST EXPLICITLY DEFINE A TYPE)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let (my_name, my_age) = ("James", "22");
    println!("{} is {}", my_name, my_age);
}