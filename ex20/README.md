# Crates
See https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html 
and https://doc.rust-lang.org/cargo/ 

In Rust, release profiles are predefined, customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code. Each profile is configured independently of the others.

Cargo has two main profiles: the dev profile Cargo uses when you run cargo build, and the release profile Cargo uses when you run cargo build --release. The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.

## Changes in profile

For the default profiles there are sensible defaults, but these can be changed in the Cargo.toml file.
The example below show the default settings for opt-level (optimalization level range 0-3) for the dev and the release profile.

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Documentation

In [lib.rs](src/lib.rs) there is an example of code with documentation included.

```rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Cargo can be used to document you crates for use by other programmers.

To generate and open documentation use

```bash
cargo doc --open
```

To only generate documentation use
```bash
cargo doc
```
The documentation will be available in <projectdirectory>/target/doc/ex20/all.html

## Testing
Documentation may contain test which are executed when running ```cargo test```.

Example:

```bash
$ cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/ex20-baa02e0178587043)

running 1 test
test tests::test_add_one ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/ex20-b85faf87b758012d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ex20

running 4 tests
test src/lib.rs - utils::mix (line 85) ... ok
test src/lib.rs - add_one (line 10) ... ok
test src/lib.rs - utils::mix (line 76) ... ok
test src/lib.rs - add_two (line 23) ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.12s; merged doctests compilation took 0.11s
```

Besides the normal **tests** also **Doc-tests** are run.

## Scope of exported elements

It is possible to re-export exports to make it easier for the user of your crate to use that without the need to change the internal structure of a crate.

Example is shown in [main.rs](src/main.rs) and [lib.rs](src/lib.rs). 
lib.rs re-exports 

## Publishing crates
See https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html

Your crate will need a unique name. While you’re working on a crate locally, you can name a crate whatever you’d like. However, crate names on crates.io are allocated on a first-come, first-served basis. Once a crate name is taken, no one else can publish a crate with that name. Before attempting to publish a crate, search for the name you want to use. If the name has been used, you will need to find another name and edit the name field in the Cargo.toml file under the [package] section to use the new name for publishing, like so:

Example:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```

### Publishing a New Version of an Existing Crate
When you’ve made changes to your crate and are ready to release a new version, you change the version value specified in your Cargo.toml file and republish. Use the Semantic Versioning rules to decide what an appropriate next version number is, based on the kinds of changes you’ve made. Then, run cargo publish to upload the new version.

### Deprecating Versions from Crates.io
Although you can’t remove previous versions of a crate, you can prevent any future projects from adding them as a new dependency. This is useful when a crate version is broken for one reason or another. In such situations, Cargo supports yanking a crate version.

Yanking a version prevents new projects from depending on that version while allowing all existing projects that depend on it to continue. Essentially, a yank means that all projects with a Cargo.lock will not break, and any future Cargo.lock files generated will not use the yanked version.

A yank does not delete any code. It cannot, for example, delete accidentally uploaded secrets. If that happens, you must reset those secrets immediately.

For example, if we’ve published a crate named guessing_game version 1.0.1 and we want to yank it, then we’d run the following in the project directory for guessing_game:

```bash
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1
```