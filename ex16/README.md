# Packageing

See https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html 

There are different methods for packaging:
- Crates, A tree of modules that produces a library or executable
- Packages, A Cargo feature that lets you build, test, and share crates
- Modules and use, Let you control the organization, scope, and privacy of paths
- Paths, A way of naming an item, such as a struct, function, or module
- Workspaces, for very large projects a set of interrelated packages that evolve together

## Crates
A crate is the smallest amount of code that the Rust compiler considers at a time. Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.

There are two forms or crates:
- a binary crate, this can be executed and must have a main function.
- library crate, define functionality which can be used by multiple projects.

### Binary
Binary crates are programs you can compile to an executable that you can run, such as a command line program or a server. Each must have a function called main that defines what happens when the executable runs.

### Library
Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. 

## Packages
A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.

## Modules
Modules let us organize code within a crate for readability and easy reuse. Modules also allow us to control the privacy of items because code within a module is private by default. Private items are internal implementation details not available for outside use. We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.

## Paths
A Path is an expression to find an item in the module tree.

A path can take two forms:
- An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
- A relative path starts from the current module and uses self, super, or an identifier in the current module.