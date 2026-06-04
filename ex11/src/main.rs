// #![allow(warnings)] Hint the compiler to ignore warnings in this file. This is useful when you are still learning and want to focus on the main concepts without being distracted by warnings. However, it's generally not recommended to use this in production code, as it can hide potential issues.
fn main() {
    println!("Hello, world!");

    logical_operators();
    repeating_operators();
}

fn logical_operators() {
    let x = 5;
    let y = 10;

    if x < y {
        println!("x: {} is less than y: {}", x, y);
    } else {
        println!("x: {} is greater than or equal to y: {}", x, y);
    }

    let age: u16 = 18;
    let is_adult = if age >= 18 { true } else { false };

    println!("The person with age: {} is an adult? {}", age, is_adult);

    let number = 6;
    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn repeating_operators() {
    loop_operator();
    while_operator();
    for_operator();
}

fn loop_operator() {
    let mut count = 0;

    let result = loop {
        count += 1;
        println!("Count: {}", count);

        if count >= 20 {
            break count;
        }
    };

    println!("Exited the loop with count: {}", result);

    let mut count = 0;
    'counting_up: loop {
        // 'counting_up is a label for the loop, which allows us to break out of it from within nested loops.
        println!("Count: {}", count);
        let mut remaining = 10;

        loop {
            println!("Remaining: {remaining} Count: {count}");
            if remaining == 6 {
                println!("Breaking out of the inner loop remaining == {remaining}");
                break;
            }
            if count == 5 {
                println!(
                    "Breaking out of the outer loop count == {count} remaining == {remaining}"
                );
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End nested loop with count: {}", count);
}

fn while_operator() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_operator() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("READY!");
}
