use std::arch::x86_64::_XCR_XFEATURE_ENABLED_MASK;

fn main() {
    println!("Hello, world!");
    hello_world();

    let height = 123;
    tell_height(height);

    human_id("Alice", 30, 165.5);

    let _x = {
        let price: u32 = 5;
        let qty: u32 = 10;
        price * qty
    };
    println!("The total is {}.", _x);

    println!("The sum of 5 and 10 is {}.", add(5, 10));

    let z: fn(a: i32, b: i32) -> i32 = add;
    println!("The sum of -5 and 15 is {}.", z(-5, 15));

    let weight = 78.0; // in kg
    let height = 1.78; // in meters
    let bmi = calculate_bmi(weight, height);
    println!("My BMI for {} kg and {} m is {:.2}.", weight, height, bmi);
}

fn hello_world() {
    println!("Hello, \u{1F980} Rust!");
}

fn tell_height(height: u32) {
    println!("My height is {} cm.", height);
}

fn human_id(name: &str, age: u8, height: f32) {
    println!(
        "My name is {}, I'm {} years old and {} cm tall.",
        name, age, height
    );
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// BMI
fn calculate_bmi(weight: f32, height: f32) -> f32 {
    weight / (height * height)
}
