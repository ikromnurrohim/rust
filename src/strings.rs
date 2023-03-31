// Primitive str = Imutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structur - Use wheen you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");
    // Get length
    println!("Length : {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    println!("{:?}", (hello));

    // Capacity in bytes
    println!("Capacity is : {}", hello.capacity());

    // Check if empty
    println!("Is empty : {}", hello.is_empty());

    // Contains
    println!("Contains 'World' : {}", hello.contains("World"));

    // Replace
    println!("Replace 'World' : {}", hello.replace("World", "There"));

    // Loop through string by withspace
    for word in hello.split(" "){
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    // Assertion testing (left == right)
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);


}