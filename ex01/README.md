# Hello world

Create a rust program: hello.rs with content

```rust
fn main() {
    println!("Hello, world!");
}
```

To use it, compile the code ```rustc hello.rs``` this produces hello as an executable file.

One can also use the cargo package manager.

```bash
cargo new helloProject
    Creating binary (application) `helloProject` package
warning: package name `helloProject` is not snake_case or kebab-case which is recommended for package names, consider `helloproject`
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

Cargo created the project

```bash
ls -R helloProject/
helloProject/:
Cargo.toml  src

helloProject/src:
main.rs
```

The project is described by [Cargo.toml](./helloProject/Cargo.toml).


```bash
bvpelt@uranus:~/Develop/rusttutorial/ex01$ cd helloProject/
bvpelt@uranus:~/Develop/rusttutorial/ex01/helloProject$ cargo run
   Compiling helloProject v0.1.0 (/home/bvpelt/Develop/rusttutorial/ex01/helloProject)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/helloProject`
Hello, RUST from CARGO!
```