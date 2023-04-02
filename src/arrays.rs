// Arrays - Fixed list where the elements are same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // Re-assign value
    numbers[0] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[1]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Array are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}