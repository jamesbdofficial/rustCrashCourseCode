// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run(){
    // Must be declared as mutable if you want to re-assign a value
    // Has to be exactly the same amount of elements as declared.
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let numbers1: [i32; 4] = [1, 2, 3, 4];

    // Re-assign value
    numbers[4] = 6;

    println!("{:?}", numbers);
    
    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get  array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    println!("Array occupies {} bytes", mem::size_of_val(&numbers1));

    // Get slice
    let big_slice: &[i32] = &numbers;
    println!("Slice: {:?}", big_slice);
    
    let small_slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", small_slice);
}