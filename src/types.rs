/*
Primitive Types --
Integers : u8, i8, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Float :  f32, f64
Boolean : bool
Character : char
Tupples
Array
 */

/* Rust is a statically typed language, which mean that it must now
type of all variable at compile time, hover, the compiler can infer what type we want to use
based on the value and how we use it.
*/
pub fn run(){
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 112142321222;

    // Find max size
    println!("Max i32 : {}", std::i32::MAX);
    println!("Max i64 : {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let greater_than: bool = 10 < 5;

    // Character (is just single unicode character)
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, greater_than, a1, face));
}
