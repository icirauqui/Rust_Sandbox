// Arrays - Fixed list where elements are the same data types

// Remove std when using mem
//use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // Print the whole array
    println!("{:?}", numbers);

    // Print single values
    println!("Single value: {}", numbers[0]); // Starting at 0 !!!!

    // Change value (make array mutable)
    // Cannot add elements but can change them
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Get array length
    println!("Array length: {}", numbers.len());
    
    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice 
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);
}