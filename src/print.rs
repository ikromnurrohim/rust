pub fn run(){
    // Print to console
    println!("Print from print.rs file");

    // Basic formating
    println!("Number is {}", 1);

    // Positional arguments
    println!("{0} is from {1} and {0} like to {2}", "Ikrom", "Tegal", "code");

    // Named arguments
    println!("{name} like to {activity}", name = "Ikrom", activity = "code");

    // Placeholder trait
    println!("Binary : {:b} Hex : {:x} Octal : {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (10, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
