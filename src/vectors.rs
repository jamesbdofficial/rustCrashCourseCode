// Vectors - Resizable arrays

use std::mem;

pub fn run(){
    // Must be declared as mutable if you want to re-assign a value
    // Has to be exactly the same amount of elements as declared.
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[4] = 6;

    // Add on to vector
    numbers.push(7);
    numbers.push(8);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);
    
    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get  Vector length
    println!("Vector Length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let big_slice: &[i32] = &numbers;
    println!("Slice: {:?}", big_slice);
    
    let small_slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", small_slice);

    // Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
    println!("Each value has been doubled in the vector")
}