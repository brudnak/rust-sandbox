pub fn run() {
    let mut vehicle = String::from("Tesla");

    // Get length
    println!("Length: {}", vehicle.len());

    // Push char
    vehicle.push(' ');

    // Push string
    vehicle.push_str("Model X");

    // Capacity in bytes
    println!("Capacity: {}", vehicle.capacity());

    //Check if empty
    println!("Is Empty: {}", vehicle.is_empty());

    //Contains
    println!("Contains 'Tesla' {}", vehicle.contains("Tesla"));

    // Replace
    println!("Replace: {}", vehicle.replace("X", "S"));

    // Loop through string by whitespace
    for word in vehicle.split_whitespace() {
        println!("{}", word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{}", vehicle);
}
