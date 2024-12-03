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

## 3. Propagating Errors

You can propagate errors in Rust using the ? operator. This operator returns early if the result is an Err or None and propagates the error to the calling function.

**Example: Propagating Errors with ?**

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() -> Result<(), String> {
    let result = divide(10, 0)?; // This will return the error if divide fails
    println!("Result: {}", result);
    Ok(())
}
```

**Explanation:**

- The `?` operator automatically returns the `Err` from the divide function if it occurs. The main function returns a Result to propagate the error.
- The return type of main is `Result<(), String>`, where `()` means no meaningful return value on success.

## 4. Custom Error Types

You can create custom error types using enums or structs, especially when your application needs to deal with multiple types of errors.

**Example: Custom Error Type**

```rust
use std::fmt;

#[derive(Debug)]
enum MyError {
    DivisionByZero,
    NegativeNumber,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MyError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MyError::NegativeNumber => write!(f, "Negative number is not allowed"),
        }
    }
}

fn divide(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 {
        Err(MyError::DivisionByZero)
    } else if a < 0 || b < 0 {
        Err(MyError::NegativeNumber)
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(-10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

**Explanation:**

- We define a custom error type `MyError` with variants like DivisionByZero and NegativeNumber.
- We implement `fmt::Display` for MyError so we can easily print the error messages.
- The divide function returns a `Result<i32, MyError>`, and the error type is used to communicate specific error conditions.
