use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Ikrom";
    let status = "100%";
    // println!("Args is, {:?}", command);

    if command == "hello" {
        println!("Hi, {} how are you?", name);
    }else if command == "status" {
        println!("Status: {}", status);
    }else {
        println!("That is invalid command");
    }


}