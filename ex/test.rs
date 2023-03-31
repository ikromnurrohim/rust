// fn
// main(){
//    println!("Rust says Hello to TutorialsPoint !!");
//    println!(" hello");
//    println!("format {} arguments {}", "some", "in console");
// }

/*
fn keyword used to defined function,
The main() is a predefined function that acts as an entry point to the program.
println! is predefined macro in rust. Macro calls are always marked with an exclamation mark – !.
*/


// fn
// main() {
//    let company_string = "company";     // string type
//    let rating_float = 4.5;             // float type
//    let is_growing_boolean = true;      // boolean type
//    let icon_char = '❤';               //unicode character type

//    println!("company name is {} ", company_string);
//    println!("company rating is {} ", rating_float);
//    println!("company is gworing {}", is_growing_boolean);
//    println!("company icon is {}", icon_char);
// }

/*
Rust is a statically typed language. Every value in Rust is of a certain data type. The compiler can automatically infer data type of the variable based on the value assigned to it.
The println! macro take two arguments :
- A special syntax { }, which is the placeholder
- The variable name or a constant

The placeholder will be replace by variable's value
*/

/*
Rules for naming variable
- The name of varibale can be composed of letters, digits, and underscore character.
- It must begin with either a letter or an underscore
- Upper and lowercase letters are distinct because Rust is case-sensitive.

Syntax
The data type is optional while declaring a variable in Rust. The data type is inferred from value assigned to the variable.
example
let variable_name = value;          // no type specified
let variable_name:dataType = value; // type specified

Immutable
By default, variables are immutable - read only in Rust. In other words, the variable's value cannot be changed once value is bound to a variable name.
example :
code below will be error
fn main() {
   let fees = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
}

Mutable
Variables are immutable by default. Prefix the variable name with mut keyword to make mutable. The value of mutable can be changed.
example :
let mut variable_name = value;
let mut variable_name:dataType = value;
Let us understand this with an example

fn main() {
   let mut fees:i32 = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
}
*/


/*
Constant
Constant represent value that cannot be changed. If you declare a constant then there is no way its value changes. The keyword for using constants is const. Constants must be explicitly typed.
example declaring constant variable :
const VARIABLE_NAME = value;

Rust Constant Naming Convention
The naming convention for Constants are similar to that of variables. All characters in a constant name are usually in uppercase. Unlike declaring variables, the let keyword is not used to declare a constant.

We have used constants in Rust in the example below −
fn main() {
   const USER_LIMIT:i32 = 100;    // Declare a integer constant
   const PI:f32 = 3.14;           //Declare a float constant

   println!("user limit is {}",USER_LIMIT);  //Display value of the constant
   println!("pi value is {}",PI);            //Display value of the constant
}

Constants v/s Variables

In this section, we will learn about the differentiating factors between constants and variables.

    Constants are declared using the const keyword while variables are declared using the let keyword.

    A variable declaration can optionally have a data type whereas a constant declaration must specify the data type. This means const USER_LIMIT=100 will result in an error.

    A variable declared using the let keyword is by default immutable. However, you have an option to mutate it using the mut keyword. Constants are immutable.

    Constants can be set only to a constant expression and not to the result of a function call or any other value that will be computed at runtime.

    Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of the code need to know about.


Shadowing of Variables and Constants
Rust allows programmers to declare variables with the same name. In such a case, the new variable overrides the previous variable.
Let us understand this with an example.
fn main() {
   let salary = 100.00;
   let salary = 1.50 ;
   // reads first salary
   println!("The value of salary is :{}",salary);
}
The above code declares two variables by the name salary. The first declaration is assigned a 100.00 while the second declaration is assigned value 1.50. The second variable shadows or hides the first variable while displaying output.

Rust supports variables with different data types while shadowing.
Consider the following example.
The code declares two variables by the name uname. The first declaration is assigned a string value, whereas the second declaration is assigned an integer. The len function returns the total number of characters in a string value.
fn main() {
   let uname = "Mohtashim";
   let uname = uname.len();
   println!("name changed to integer : {}",uname);
}


Unlike variables, constants cannot be shadowed. If variables in the above program are replaced with constants, the compiler will throw an error.
fn main() {
   const NAME:&str = "Mohtashim";
   const NAME:usize = NAME.len();
   //Error : `NAME` already defined
   println!("name changed to integer : {}",NAME);
}

*/



fn main(){
   let name:String = String::from("TutorialsPoint");
   println!("{}", name);
   display(name);
   //cannot access name after display
}
fn display(param_name:String){
   println!("param_name value is :{}",param_name);
}

