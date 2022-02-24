// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let array1 = [1,2,3];
    let array2 = array1;
    println!("Values: {:?}", (array1, array2));

    // With non-primitives, if you assign another variable to a piece of data, 
    // the first variable will not longer hold that value. You'll need to use a 
    // reference (&) to point to the resource

    // Vector
    let vector1 = vec![4, 5, 6];
    let vector2 = &vector1;
    //println!("Values: {:?}", (vector1, vector2)); // GIVES ERROR - 'value used here after move' on 'vector1' when & is not used.
    println!("Values: {:?}", (&vector1, vector2));

}