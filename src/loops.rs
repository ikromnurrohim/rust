// Loops - Used for iterate untill a condition is met.

pub fn run() {
    let mut count = 0;

    // Infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);
    //
    //     if count == 20 {
    //         break;
    //     }
    // }

    // While loop (FizzBuzz)
    // while count <= 50 {
    //     if count % 15 == 0 {
    //         println!("FizzBuzz");
    //     }else if count % 3 == 0{
    //         println!("Fizz");
    //     }else if count % 5 == 0{
    //         println!("Buzz");
    //     }else {
    //         println!("{}", count);
    //     }
    //     // Inc
    //     count += 1;
    // }

    // For Range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        }else if x % 3 == 0{
            println!("Fizz");
        }else if x % 5 == 0{
            println!("Buzz");
        }else {
            println!("{}", x);
        }
    }
}
