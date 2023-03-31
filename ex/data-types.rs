// Rust is a statically typed language. Every value in Rust is of a certain data type. The compiler can automatically infer data type of the variable based on the value assigned to it.

fn main() {
   let company_string = "TutorialsPoint";  // string type
   let rating_float = 4.5;                 // float type
   let is_growing_boolean = true;          // boolean type
   let icon_char = '♥';                    //unicode character type

   println!("company name is:{}",company_string);
   println!("company rating on 5 is:{}",rating_float);
   println!("company is growing :{}",is_growing_boolean);
   println!("company icon is:{}",icon_char);
}
