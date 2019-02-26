/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

pub fn run() {
    // Default is "i32"
    let _x = 1;

    // Default "f64"
    let _y = 2.5;

    // Add explicit type
    let _z: i64 = 9223372036854775807; // max value for 64-bit signed

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let _is_active: bool = true;

    // Get boolean from expression
    let is_greater = 10 < 5;

    let a1 = 'a';
    let crab = '\u{1F980}';

    println!("{:?}", (_x, _y, _z, is_greater, a1, crab));
}
