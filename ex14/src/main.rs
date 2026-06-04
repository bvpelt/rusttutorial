/*
 * This code defines two generic enums, `Option` and `Result`, which are commonly used in Rust for handling optional values and error handling, respectively.
 * The `main` function demonstrates how to use the `divide` function, which returns an `Option<f64>` indicating whether the division was successful or if it resulted in an error (division by zero).

enum Option<T> {
    // Defining a generic Option type
    Some(T), // A value. if something is positive/present it returns this type else none will be returned
    None,    // Absence of a value.
}

enum Result<T, E> {
    // Defining a generic Result type
    Ok(T),  // Represents a value
    Err(E), // Represents an error
}
*/

fn main() {
    println!("Hello, world!");

    test_option(2.0); // Should print the result of 10.0 / 2.0
    test_option(0.0); // Should indicate that division by zero is not possible
    test_result(21.0); // Should print the result of 10.0 / 2.0
    test_result(0.0); // Should indicate that division by zero is not possible
}

fn test_option(denominator: f64) {
    let result: Option<f64> = divide_option(10.0, denominator);
    println!("Result: {:?}", result);
    match result {
        Some(value) => println!("Division result: {:?}", value),
        None => println!("Cannot divide by zero!"),
    }
}

fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn test_result(denominator: f64) {
    let result: Result<f64, String> = divide_result(120.0, denominator);
    println!("Result: {:?}", result);
    match result {
        Ok(value) => println!("Division result: {:?}", value),
        Err(error) => println!("Error: {}", error),
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(numerator / denominator)
    }
}
