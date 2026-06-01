# Functions

Rust expects to have only one main function as the entry point of a program.

If the only file (main.rs) in the project (ex04) contains:
```rust
fn hello() {
    println!("Hello, world!");
}
```
The code cannot compile/execute since there is no main function.
```bash
cargo run main.rs
   Compiling ex04 v0.1.0 (/home/bvpelt/Develop/rusttutorial/ex04)
error[E0601]: `main` function not found in crate `ex04`
 --> src/main.rs:3:2
  |
3 | }
  |  ^ consider adding a `main` function to `src/main.rs`

For more information about this error, try `rustc --explain E0601`.
error: could not compile `ex04` (bin "ex04") due to 1 previous error
```

Functions return
- the result of an expression
- a value

