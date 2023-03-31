// String

// The string data type in rust can be classified into the following
// - String literal(&str)
// - String object(String)

// String Literal
// String literals (&str) are used when the value of a string is known at compile time. String literals are a set of characters, which are hardcoded into a variable. For example, let company="Tutorials Point". String literals are found in module std::str. String literals are also known as string slices.

// The following example declares two string literals − company and location.

fn string_literal() {
   let company:&str="TutorialsPoint";
   let location:&str = "Hyderabad";
   println!("company is : {} location :{}",company,location);
}

// String literals are static by default. This means that string literals are guaranteed to be valid for the duration of the entire program. We can also explicitly specify the variable as static as shown below −

fn string_static() {
   let company:&'static str = "TutorialsPoint";
   let location:&'static str = "Hyderabad";
   println!("company is : {} location :{}",company,location);
}
// The above program will generate the following output −

// company is : TutorialsPoint location :Hyderabad


// String Object
// The String object type is provided in Standard Library. Unlike string literal, the string object type is not a part of the core language. It is defined as public structure in standard library pub struct String. String is a growable collection. It is mutable and UTF-8 encoded type. The String object type can be used to represent string values that are provided at runtime. String object is allocated in the heap.

// Syntax
// To create a String object, we can use any of the following syntax −

// String::new()
// The above syntax creates an empty string

// String::from()
// This creates a string with some default value passed as parameter to the from() method.

The following example illustrates the use of a String object.

fn string_object(){
   let empty_string = String::new();
   println!("length is {}",empty_string.len());

   let content_string = String::from("TutorialsPoint");
   println!("length is {}",content_string.len());
}
// The above example creates two strings − an empty string object using the new method and a string object from string literal using the from method.

// The output is as shown below −

// length is 0
// length is 14

