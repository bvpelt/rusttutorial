# Shadowing

If a variable is declared multiple times the last declaration is used.

Example

```rust
let x:i32 = 5;      // x = 5
let x:i32 = x + 1;  // x = 6

// changing the type of a variable using shadowing
let spaces = "    ";           // type is string
let spaces = spaces.len();     // type is number
```