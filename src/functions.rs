// Functions - Used to store blocks of code to re-use

pub fn run() {
    greeting("Hello", "Ikrom");
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);

    // Bind functions values to variable
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Clouser
    let n3 = 10;
    let add_sum = |n1: i32, n2: i32 | n1 + n2 + n3;
    println!("Clouser Sum: {}", add_sum(3, 3));
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}