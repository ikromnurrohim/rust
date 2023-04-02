pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let know_person_of_age = true;

    // If Else
    if age >= 21 && check_id || know_person_of_age{
        println!("Bartender: Would you like to drink ?");
    }else if age < 21 && check_id{
        println!("Bartender: Sorry, You have to leave");
    }else{
        println!("Bartender: I'll need see you ID.");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}