/*
An Array is homogeneous of values. simply put, an array is a collection of values of the same data type.

Features of an Array
The features of an array are as listed below −
- An array declaration allocates sequential memory blocks.
- Arrays are static. This means that an array once initialized cannot be resized.
- Each memory block represents an array element.
- Array elements are identified by a unique integer called the subscript/ index of the element.
- Populating the array elements is known as array initialization.
- Array element values can be updated or modified but cannot be deleted.

Declaring and Initializing Arrays
Use the syntax given below to declare and initialize an array in Rust.

Syntax 1
let variable_name = [value1, value2, value3];

Syntax 2
let variable_name:[dataType;size] = [value1, value2, value3];

Syntax 3
let variable_name:[dataType;size] = [default_value_for_elements, size];

In the first syntax, type of the array is inferred from the data type of the array’s first element during initialization.

The following example explicitly specifies the size and the data type of the array. The {:?} syntax of the println!() function is used to print all values in the array. The len() function is used to compute the size of the array.

*/

fn example_one(){
    let arr:[i32;4] = [10,20,30,40];
    println!("{:?}", arr);
    println!("lef of array is {}", arr.len());
}


// array without data type
fn example_two(){
    let arr = [10,20,30,40];
    println!("{:?}", arr);
    println!("len of array is {}", arr.len());
}

//  example array creates and initialize all its element with a default value of -1.
fn example_three(){
    let arr:[i32;4] = [-1;4];
    println!("{:?}", arr);
    println!("len of array is {}", arr.len());
}

// array with loop
fn example_four(){
    let arr:[i32;4] = [10,20,30,40];
    println!("{:?}", arr);
    println!("len of array is {}", arr.len());

    for index in 0..arr.len(){
        println!("index is: {} & value is: {}", index, arr[index]);
    }
}


// array with iter() function
fn example_five(){
    let arr:[i32;4] = [10,20,30,40];
    println!("{:?}", arr);
    println!("len of array is {}", arr.len());

    for val in arr.iter(){
        println!("arr value is: {}", val);
    }
}


// mutable array
fn example_six(){
    let mut arr:[i32;4] = [10,20,30,40];
    arr[3] = 100;
    println!("{:?}", arr);
}


// array as parameters to function (pass by value)
fn example_seven(){
    let arr = [10,20,20];
    update_one(arr);
    println!("Inside main {:?}", arr);
}

fn update_one(mut arr:[i32;3]){
    for i in 0..arr.len(){
        arr[i] = 0;
    }
    println!("Inside update {:?}", arr);
}

// array as parameters to function (pass by reference)
fn example_eight(){
    let mut arr = [10,20,20];
    update_two(&mut arr);
    println!("Inside main {:?}", arr);
}

fn update_two(arr:&mut [i32;3]){
    for i in 0..arr.len(){
        arr[i] = 0;
    }
    println!("Inside update {:?}", arr);
}


// array declaration and constants

fn example_nine(){
    let N: usize = 20;
    let arr = [0, N]; //Error: non-constant used with constant
    println!("{:?}", arr[10]);
}


fn main() {
   const N: usize = 20;
   // pointer sized
   let arr = [0; N];

   print!("{:?}",arr);
}

//The value of an identifier prefixed with the const keyword is defined at compile time and cannot be changed at runtime. usize is pointer-sized, thus its actual size depends on the architecture you are compiling your program for.
