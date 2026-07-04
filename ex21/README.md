# Workspace

A workspace is a set of packages that share the same Cargo.lock and output directory.

We’ll have a workspace containing a binary and two libraries. The binary, which will provide the main functionality, will depend on the two libraries. One library will provide an add_one function and the other library an add_two function.

```bash
# Package containing the binary
$ cargo new adder
    Creating binary (application) `adder` package
      Adding `adder` as member of workspace at `/home/bvpelt/Develop/rusttutorial/ex21`
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

# Package containing add_one
$ cargo new add_one --lib
    Creating library `add_one` package
      Adding `add_one` as member of workspace at `/home/bvpelt/Develop/rusttutorial/ex21`
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

Bouw de binary in de workspace

```bash
$ cargo build
   Compiling add_one v0.1.0 (/home/bvpelt/Develop/rusttutorial/ex21/add_one)
   Compiling adder v0.1.0 (/home/bvpelt/Develop/rusttutorial/ex21/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s

# Run binary
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!

# Running tests
$ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/add_one-ea53099ed6660448)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-08788bd4ccd7568e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


# Only test a particular crate for instance add_one
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-ea53099ed6660448)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

# Run main function adder
$ cargo run
   Compiling adder v0.1.0 (/home/bvpelt/Develop/rusttutorial/ex21/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
Hello, world! 10 plus two is 12!

```