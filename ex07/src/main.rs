fn main() {
    println!("Hello, world!");

    let a: i32 = 5;
    println!("The value of a is: {}", a);

    // a = 10; // This will cause a compile-time error because `a` is immutable by default

    let mut b: i32 = 10; // `b` is mutable
    println!("The value of b is: {}", b);
    b = 20; // This is allowed because `b` is mutable
    println!("The value of b is now: {}", b);
}
