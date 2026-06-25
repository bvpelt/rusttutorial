# Testing

## Create test
To create a function add with tests in a library

```bash
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

## Run test

```bash
$ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/adder-49bd30106ab09ca0)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

Tests run in multiple threads by default. To use only one thread use

```bash
$ cargo test -- --test-threads=1
```

Make sure you don't depend on shared resources for your tests.

In tests print statements are not shown on success, they are shown on error by default.
To change this behaviour use:

```bash
$ cargo test -- --show-output
```

To run a single specific test use:

```bash
$ cargo test it_works_generic
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/adder-49bd30106ab09ca0)

running 1 test
test tests::it_works_generic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

```

To run a number of test matching a partial string use:

```bash
$ cargo test it
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/adder-49bd30106ab09ca0)

running 3 tests
test tests::it_works ... ok
test tests::it_adds_two ... FAILED
test tests::it_works_generic ... ok

failures:

---- tests::it_adds_two stdout ----

thread 'tests::it_adds_two' (41066) panicked at src/lib.rs:119:9:
assertion `left == right` failed
  left: 5
 right: 4
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`


$ cargo test can
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/adder-49bd30106ab09ca0)

running 3 tests
test tests::larger_can_hold_smaller ... ok
test tests::larger_can_hold_smaller_rect ... ok
test tests::smaller_cannot_hold_larger ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s


```

To run tests which are marked as ignored, for instance because they take a long time use:

```bash
$ cargo test -- --ignored
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/adder-49bd30106ab09ca0)

running 1 test
test tests::expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

Tests can be placed in seperate directorie test under project root and use own crate.

When using directory structure

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_tests.rs
```

To run the integration_tests use:

```bash
$ cargo test --test integration_tests
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running tests/integration_tests.rs (target/debug/deps/integration_tests-300d4f11f823fb2d)

running 1 test
test add_five_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

Common functionality

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

```bash
$ cargo test --test integration_tests -- --show-output
   Compiling adder v0.1.0 (/home/bvpelt/Develop/rusttutorial/ex18/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running tests/integration_tests.rs (target/debug/deps/integration_tests-300d4f11f823fb2d)

running 1 test
test add_five_test ... ok

successes:

---- add_five_test stdout ----
== setup common ==


successes:
    add_five_test

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```