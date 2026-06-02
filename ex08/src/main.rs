const SECONDS_IN_MINUTE: u32 = 60; // This is a valid constant declaration.
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * 60; // This is also valid, as it uses another constant in its declaration.
const SECONDS_IN_DAY: u32 = SECONDS_IN_HOUR * 24; // This is valid as well, as it uses another constant in its declaration.
const SECONDS_IN_WEEK: u32 = SECONDS_IN_DAY * 7; // This is valid as well, as it uses another constant in its declaration.

fn main() {
    println!("Hello, world!");

    //const mut a: i32 = 5; // This line will cause a compile-time error because `const` variables cannot be mutable.

    const PI: f64 = 3.14159; // This is a valid constant declaration.

    println!("The value of PI is: {}", PI);
    println!("The value of SECONDS_IN_MINUTE is: {}", SECONDS_IN_MINUTE);
    println!("The value of SECONDS_IN_HOUR is: {}", SECONDS_IN_HOUR);
    println!("The value of SECONDS_IN_DAY is: {}", SECONDS_IN_DAY);
    println!("The value of SECONDS_IN_WEEK is: {}", SECONDS_IN_WEEK);
}
