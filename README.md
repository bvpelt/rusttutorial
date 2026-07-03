# Rusttutorial
Rust tutorial

See: https://www.youtube.com/watch?v=rQ_J9WH6CGk

RUST is a systems programming languages, that is a compiled language.

RUST is popular by developers.
RUST is a memory save programming language by design.
RUST offers balance in:
- Speed vs Safety
- Concurrency vs Portability

**Speed**
Compiles to machine code, close to hardware, thereby very fast. Comparable to C/C++.

**Safety**
Safe in memory management. Rust uses ownership and borrowing to provide safe memory management.

**Concurrency**
Multiple threads of execution in parallel.

**Portability**
Compile once, run everywhere.

RUST has a package manager **Cargo**. Like npm for node or pip for python.

Reference site: [https://rust-lang.org](https://rust-lang.org).


- [Learn rust](https://rust-lang.org/learn/)
- [Watch rust videos](https://www.youtube.com/channel/UCaYhcUwRBNscFNUKTjgPFiA)
- [Standard RUST library](https://doc.rust-lang.org/std/index.html)

The claims rust make may be true, but in order to fullfill these claims the programmer must adhere to all rust concepts like:
- ownership
- borrowing

The learning curve is steep and takes a long time.

# Installation
To install RUST on linux (UBUNTU) use
```bash
sudo apt update && sudo apt upgrade -y
sudo apt install build-essential curl git -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
```


## Check installation

```bash
# Check the toolchain manager
rustup --version
rustup 1.29.0 (28d1352db 2026-03-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: the currently active `rustc` version is `rustc 1.96.0 (ac68faa20 2026-05-25)`

# Check the rust compiler
rustc --version
rustc 1.96.0 (ac68faa20 2026-05-25)

# Check cargo - the package manager
cargo --version
cargo 1.96.0 (30a34c682 2026-05-25)
```

## Extensions
The next extensions are needed:
- rust-analyzer
- Even Better TOML
- codelldb

To enable debugging add the following launch.json in the top directory .vscode

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug myrustapp",
            "cargo": {
                "args": [
                    "build",
                    "--bin",
                    "myrustapp"
                ],
                "filter": {
                    "name": "myrustapp",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

# Git version management

```bash
# Create and switch to feature branch (if not already on one)
git checkout -b feature/\<name\>

# Commit your changes
git add .
git commit -m "<description>"

# Push the feature branch
git push origin feature/\<name\>

# Create a tag for this feature
git tag -a v1.0.0 -m "<description>"

# Push the tag
git push origin v1.0.0
```

# Statements
- Function and variable names should be written in [snake case](https://en.wikipedia.org/wiki/Snake_case).
- The order of main and other functions is free. No need to first define functions and then main. (This is called Hoisting).
- Expressions return a value, statments do not return a value.
- global variables should be declared const or static.

# Memory concepts
- Ownership
- Borrowing
- References

Some programmaing languages let you control memory allocation/deallocation (C, C++). This can cause issues by releasing memory not at all, or more than once.

Other programming languages try to solve this problem by using a garbage collector (Java). This introduces a problem that when the garbage collector is running the program freezes. 

Every value has a single owner.

Rules with regard to ownership:
- Each value in Rust has one owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

# Logging
Conform to opentelemetry since a lot of tooling is already available.


# Index

- [Initial / basic](ex01/README.md) [repository](ex01)
- [Guessing game](guessing/README.md) [repository](guessing)
- [Primitive datatypes](ex02/README.md) [repository](ex02)
- [Compound datatypes](ex03/README.md) [repository](ex03)
- [Functions](ex04/README.md) [repository](ex04)
- [Ownership](ex05/README.md) [repository](ex05)
- [Borrowing and References](ex06/README.md) [repository](ex06)
- [Variables & mutability](ex07/README.md) [repository](ex07)
- [Constants](ex08/README.md) [repository](ex08)
- [Shadowing](ex09/README.md) [repository](ex09)
- [Comment](ex10/README.md) [repository](ex10)
- [Control flow](ex11/README.md) [repository](ex11)
- [Structs](ex12/README.md) [repository](ex12)
- [Enums](ex13/README.md) [repository](ex13)
- [Error Handling](ex14/README.md) [repository](ex14)
- [Collection Types](ex15/README.md) [repository](ex15)
- [Packaging](ex16/README.md) [repository](ex16)
- [Generics](ex17/README.md) [repository](ex17)
- [Buffer](buffer/README.md) [repository](buffer)
- [Testing](ex18/README.md) [repository](ex18)
- [Minigrep](minigrep/README.md) [repository](minigrep)
- [Closure, Iterator, I/O, optimize minigrep](ex19/README.md) [repository](ex19)
- [Crates](ex20/README) [repository](ex20)