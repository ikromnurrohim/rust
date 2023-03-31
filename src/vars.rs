// Variable hold primitive data or reference to data
// Variable are mutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Ikrom";
    let mut age = 20;
    println!("My name {} and I am {}", name, age);
    age = 21;
    println!("My name {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variable
    let ( my_name, my_age ) = ( "Ikrom", 30 );
    println!("{} is {}", my_name, my_age);

}

