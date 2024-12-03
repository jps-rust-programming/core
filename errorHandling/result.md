# Error handling in Rust

In Rust, error handling is a fundamental part of the language, and it is done using two primary mechanisms: `Result` and `Option` types. These types allow you to handle errors explicitly, ensuring safety and making the code more robust.

## 1. The Result Type

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

## 2. The Option Type

The Option type is used when a value is either present or absent (it’s particularly useful for handling cases where a value may not exist).

It’s defined as:

```rust
enum Option<T> {
    Some(T),  // A variant for a value of type T
    None,     // A variant representing no value
}
```

**Example: Using Option for Optional Values**

```rust
fn find_even_number(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

fn main() {
    let numbers = [1, 3, 5, 8, 9];

    match find_even_number(&numbers) {
        Some(num) => println!("Found an even number: {}", num),
        None => println!("No even number found"),
    }
}
```

**Explanation**:

- The find_even_number function searches through the array of integers. If it finds an even number, it returns Some(number). If it doesn’t find any, it returns None.
- We use match to handle both the case where a number is found (Some) and the case where no number is found (None).

#### Common Option Methods:

- `is_some(), is_none()`: Check if the option contains Some or is None.
- `unwrap()`: Returns the value inside Some, but panics if the option is None.
- `expect()`: Similar to unwrap(), but with a custom error message.
- `map(), and_then()`: Transform the value inside Some, if present.
