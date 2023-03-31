/*
Tupple is a compound data type,  A scalar type can store only one type of data, for example an i32 variable can store only a singgle integer value. in compound types, we can store more than one value at a time and it can be of different types.
Tuple have a fixed length - once declared they cannot grow or shrink in size. The tuple index start from 0.
*/

/* Syntax
let tuple_name:(data_type1, data_type2, data_type3) = (value1, value2, value3);

let tuple_name = (value1, value2, value3);
*/

fn example_one() {
    let tuple:(i32, f64, u8) = (-325, 4.5, 22);
    println!("{:?}", tuple);
}

/*
The println!("{ }",tuple) syntax cannot be used to display values in a tuple. This is because a tuple is a compound type. Use the println!("{:?}", tuple_name) syntax to print values in a tuple.
*/

fn example_two(){
    let tuple:(i32, f64, u8) = (-312, 4.5, 22);
    println!("{:?}", tuple.0);
    println!("{:?}", tuple.1);
    println!("{:?}", tuple.2);

}

// passes a tuple as parameter to a function. Tuple are passed by value to functions.

fn print_one( x:(i32,bool,f64) ){
    println!("Inside print method");
    println!("{:?}", x);
}

fn example_three(){
    let b:(i32,bool,f64) = (110, true, 10.9);
    print_one(b)
}

/*
Destructing
Destructing assignment is a feature of rust wherein we unpack the values of a tuple. This is achieved by assigning a tuple to distinct variables.
*/

fn print_two(x:(i32,bool,f64)){
    println!("Inside print_two method");
    println!("{:?}", x);
    let (age,is_male,cgpa) = x; //assign a tuple to distinct variables
    println!("Age is ? {}, is male ?{}, cgpa is {}", age, is_male, cgpa);
}

fn main(){
    let b:(i32,bool,f64) = (21, true, 10.10);
    print_two(b);
}
