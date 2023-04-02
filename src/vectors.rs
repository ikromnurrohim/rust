// Vectors - Resizeable Vector

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // Re-assign value
    numbers[0] = 20;

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    // Pop of last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[1]);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 10;
    }

    println!("Number Vec: {:?}", numbers);

}