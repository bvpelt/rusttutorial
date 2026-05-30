fn main() {
    let x: i32 = 42;
    let y: u64 = 100;

    println!("signed integer: {}", x);
    println!("unsigned integer: {}", y);

    let e32: i32 = i32::pow(2, 30) - 1 + i32::pow(2, 30); // Maximum value for i32
    let i32: i32 = -1*(i32::pow(2, 30) - 1 + i32::pow(2, 30)) ;    // Minimum value for i32
    let e64: i64 = i64::pow(2, 62) - 1 + i64::pow(2, 62); // Maximum value for i64
    let i64: i64 = -1*(i64::pow(2, 62) - 1 + i64::pow(2, 62));    // Minimum value for i64
    let e128: i128 = i128::pow(2, 126) - 1 + i128::pow(2, 126); // Maximum value for i128
    let i128: i128 = -1*(i128::pow(2, 126) - 1 + i128::pow(2, 126));    // Minimum value for i128
   

    println!("maximum i32 value: {}", e32);
    println!("minimum i32 value: {}", i32);
    println!("maximum i64 value: {}", e64);
    println!("minimum i64 value: {}", i64);
    println!("maximum i128 value: {}", e128);
    println!("minimum i128 value: {}", i128);

    let pi: f64 = 312689.0 / 99532.0; // See https://www.johndcook.com/blog/2021/03/27/smallest-fraction/
    println!("pi: {}", pi);

    let issnow: bool = true;
    println!("is it snowing? {}", issnow);

    let letter: char = 'A';
    println!("letter: {}", letter);
}