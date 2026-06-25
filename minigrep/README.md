# minigrep

Demonstration of several concepts:

- [Organizing code](../ex16)
- [Using vectors and strings](../ex02)
- [Life time](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Handling errors](../ex14)
- [Using traits and lifetimes where appropriate](../ex17)
- [Writing tests](../ex18)


## Running minigrep from visual code

In the visual code terminal from the project directory type

```bash
$ cargo run a poem.txt
   Compiling minigrep v0.1.0 (/home/bvpelt/Develop/rusttutorial/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/minigrep a poem.txt`
Args: [
    "target/debug/minigrep",
    "a",
    "poem.txt",
]
Config: Config {
    query: "a",
    file_path: "poem.txt",
}
Searching for: a
In file      : poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

## Running tests

```bash
$ cargo test
```

## Running with enhanced search function

```bash
$ cat poem.txt 
```
```text
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```
```bash
# expect one line to match
$ cargo run -- frog poem.txt
```
```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep frog poem.txt`
Args: [
    "target/debug/minigrep",
    "frog",
    "poem.txt",
]
Config: Config {
    query: "frog",
    file_path: "poem.txt",
}
Searching for: frog
In file      : poem.txt
How public, like a frog
```
```bash
# expect multiple lines to match
$ cargo run -- body poem.txt
```
```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep body poem.txt`
Args: [
    "target/debug/minigrep",
    "body",
    "poem.txt",
]
Config: Config {
    query: "body",
    file_path: "poem.txt",
}
Searching for: body
In file      : poem.txt
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```
```bash
# expect no line to match
$ cargo run -- monomorphization poem.txt
```
```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep monomorphization poem.txt`
Args: [
    "target/debug/minigrep",
    "monomorphization",
    "poem.txt",
]
Config: Config {
    query: "monomorphization",
    file_path: "poem.txt",
}
Searching for: monomorphization
In file      : poem.txt
```

After adding environment variable to make caseinsensative search possible
```bash
$ cargo run -- to poem.txt
```
```text
   Compiling minigrep v0.1.0 (/home/bvpelt/Develop/rusttutorial/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/minigrep to poem.txt`
Args: [
    "target/debug/minigrep",
    "to",
    "poem.txt",
]
Config: Config {
    query: "to",
    file_path: "poem.txt",
    ignore_case: false,
}
Searching for: to
In file      : poem.txt
Are you nobody, too?
How dreary to be somebody!
```
```bash
IGNORE_CASE=1 cargo run -- to poem.txt
```
```text

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
Args: [
    "target/debug/minigrep",
    "to",
    "poem.txt",
]
Config: Config {
    query: "to",
    file_path: "poem.txt",
    ignore_case: true,
}
Searching for: to
In file      : poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
