// A variable is a named storage that programs can manipulate. Simply put, a variable helps programs to store values. Variables in Rust are associated with a specific data type. The data type determines the size and layout of the variable's memory, the range of values that can be stored within that memory and the set of operations that can be performed on the variable.

// Rules for Naming a Variable :
// In this section, we will learn about the different rules for naming a variable.
// The name of a variable can be composed of letters, digits, and the underscore character.
// It must begin with either a letter or an underscore.
// Upper and lowercase letters are distinct because Rust is case-sensitive.


// The data type is optional while declaring a variable in Rust. The data type is inferred from the value assigned to the variable.
// The syntax for declaring a variable is given below.

let variable_name = value;            // no type specified
let variable_name:dataType = value;   //type specified

fn main() {
   let fees = 25_000;
   let salary:f64 = 35_000.00;
   println!("fees is {} and salary is {}",fees,salary);
}


// Immutable
// By default, variables are immutable − read only in Rust. In other words, the variable's value cannot be changed once a value is bound to a variable name.

// Let us understand this with an example.

fn imutable_example() {
   let fees = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
}


// Mutable
// Variables are immutable by default. Prefix the variable name with mut keyword to make it mutable. The value of a mutable variable can be changed.

// The syntax for declaring a mutable variable is as shown below −

let mut variable_name = value;
let mut variable_name:dataType = value;
Let us understand this with an example

fn mutable_example() {
   let mut fees:i32 = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
}
