pub fn run() {
    vehicle("Tesla", "Roadster");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let number_three = 10;
    let add_nums = |number_one: i32, number_two: i32| number_one + number_two + number_three;
    println!("C Sum: {}", add_nums(3, 3));
}

fn vehicle(make: &str, model: &str) {
    println!("{} {}", make, model)
}

fn add(number_one: i32, number_two: i32) -> i32 {
    number_one + number_two
}
