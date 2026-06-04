fn main() {
    println!("Hello, world!");

    let x: i32 = 5;
    let x: i32 = x + 1; // Shadowing allows us to reuse the same variable name, but with a new value.
                        // x=10; // This will cause a compile-time error because `x` is immutable by default. To fix this, we can declare `x` as mutable using `let mut x: i32 = 5;`.
    let x = x * 1000;
    {
        let x = x * 2; // This creates a new variable `x` that shadows the outer `x`. The value of this inner `x` is 12.
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");
}
