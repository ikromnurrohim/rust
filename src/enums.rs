// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up,
    Right,
    Down,
    Left,
}

fn move_avatar(m: Movement) {
    // Perfoming action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Right => println!("Avatar moving right"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
    }
}

pub fn run() {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Left;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}