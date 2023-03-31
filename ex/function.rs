// function
fn hello(){
   println!("hello world");
}

// function with return statement
fn with_return_statement() -> &'static str{
   return "Hello World gaes !!";
}


// function without return statement
fn without_return_statement() -> &'static str{
   "Hello World" // no semicolon means that value isreturned
}
fn get_pi() -> f64{
   22.0/7.0
}

fn main_statement(){
   println!("main statement");
   hello();
   println!("{}\n{}\n{}", with_return_statement(), without_return_statement(), get_pi());
}

/*
function with parameters
parameters are mechanism to pass value to functions. the number of value passed to a function must match with the number of parameter defined
*/

/*
Pass by Value
When a method is invoked, a new storage location is created for each value parameter. The values of the actual parameters are copied into them. Hence, the changes made to the parameter inside the invoked method have no effect on the argument.
*/

fn mutate_to_zero(mut param_no:i32){
   param_no = param_no * 0;
   println!("param_no value is, {}", param_no);
}

/*
Pass by Reference
When you pass parameters by reference, unlike value parameters, a new storage location is not created for these parameters. The reference parameters represent the same memory location as the actual parameters that are supplied to the method. Parameter values can be passed by reference by prefixing the variable name with an & .
*/

fn mutate_to_zero_v2(param_no:&mut i32){
   *param_no = 0; //de reference
}
// The * operator is used to access value stored in the memory location that the variable param_no points to. This is also known as dereferencing.

/*
Passing string to a function
The main() function passes a string object to the display() function.
*/

fn display(param_name:String){
   println!("param_name value is, {}", param_name);
}

fn main(){
   println!("main param");
   let no:i32 = 5;
   mutate_to_zero(no);
   println!("no value is, {}", no);

   let mut no:i32 = 5;
   mutate_to_zero_v2(&mut no);
   println!("no value is, {}", no);

   let name:String = String::from("TutorialsPoint");
   display(name);
   // cannot access name after display
}
