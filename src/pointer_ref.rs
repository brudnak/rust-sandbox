pub fn run() {
    // Primitive Array
    let array_one = [1, 2, 3];
    let array_two = array_one;

    println!("Values: {:?}", (array_one, array_two));

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}