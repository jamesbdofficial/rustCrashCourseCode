/*
Primitive Types --
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take up in memory) (unsigned / regular integers)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all 
// variables at compile time, however, the compiler can usually infer what type we want
// to use based on the value and how we use it.

pub fn run(){
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 45454545454545;

    // Find max size
    println!("/------------------------------------------ Integers ------------------------------------------/");
    println!("Max i8: {}", std::i8::MAX);
    println!("Max i16: {}", std::i16::MAX);
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);
    println!("/------------------------------------------ FLOATS ------------------------------------------/");
    println!("Max f32: {}", std::f32::MAX);
    println!("Max f64: {}", std::f64::MAX);
    println!("/--------------------------------------------------------------------------------------------/");

    //Boolean
    let is_active = true;

    //Get boolean from expression
    let is_greater = 5 > 10;
    let is_greater_explicit: bool = 10 > 5;

    // Char
    let a1 = 'a';
    // let a2 = 'ab'; gives error - character literal may only contain one codepoint

    //Unicode in char
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, is_greater_explicit, a1, face));
}