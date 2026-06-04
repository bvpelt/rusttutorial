# Error handling

Errors are a fact of life in software. You need tools to handle errors.

Possible tools
- Option, used to check if a value is present
- Result, used to handle specific errors.

## Option

Example

```rust
enum Option<T> { // Defining a generic Option type
    Some(T), // A value. if something is positive/present it returns this type else none will be returned
    None,    // Absence of a value.
}
```

## Result

Example

```rust
enum Result<T, E> { // Defining a generic Result type
    Ok(T), // Represents a value
    Err(E), // Represents an error
}
```