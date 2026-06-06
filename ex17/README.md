# Generics

Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication. 

Some generic functions won't work out of the box. A number of operators work on primary datatypes, but not on compound type. The compiler will detect this at compile time and show the error. To make those generic functions work use the Rust way.

Example: 
In a struct (compound type) the generic function cannot compair. To make this work use:

```rust
 #[derive(PartialEq, PartialOrd)]  // add to tell the compiler how to compair using the first element 'x' first.
    struct Point<T> {
        x: T,
        y: T,
    }
```

Traits are simular to interface definitions which require an implementation. See [lib.rs](./src/lib.rs).