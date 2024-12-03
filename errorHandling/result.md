# Error handling in Rust

In Rust, error handling is a fundamental part of the language, and it is done using two primary mechanisms: `Result` and `Option` types. These types allow you to handle errors explicitly, ensuring safety and making the code more robust.

```rust
enum Result<T, E> {
    Ok(T), // A variant for success, with a value of type T
    Err(E), // A variant for failure, with an error of type E
}
```

- `T` represents the type of the value on success, and `E` represents the type of the error.

**Example: Handling Errors with** Result

Here’s an example of how you can handle errors with Result:

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

**Explanation:**

- The divide function returns a `Result<i32, String>`. If the division is successful, it returns `Ok(result)`, and if there is an error (like dividing by zero), it returns `Err(error_message)`.
- We then use a match statement to handle both the success (Ok) and the failure (Err).

**Common Result Methods:**

- `is_ok(), is_err()`: Check whether the result is Ok or Err.
- `unwrap()`: Returns the value inside Ok, but panics if the result is Err.
- `expect()`: Similar to unwrap(), but you can provide a custom error message.
- `map(), and_then()`: Methods for transforming the Result value.
