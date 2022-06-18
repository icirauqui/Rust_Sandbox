// Vectors - resizable arrays

// Remove std when using mem
//use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // Print the whole vector
    println!("{:?}", numbers);

    // Print single values
    println!("Single value: {}", numbers[0]); // Starting at 0 !!!!

    // Change value (make vector mutable)
    // Cannot add elements but can change them
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    // Get vector length
    println!("Vector length: {}", numbers.len());
    
    // Arrays are stack allocated
    println!("Vectors occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice 
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}