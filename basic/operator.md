## Range Operators

Rust has a special operator to create ranges.

| Operator | Description                         | Example               |
| -------- | ----------------------------------- | --------------------- |
| ..       | Range from start to end (exclusive) | 1..5 (1, 2, 3, 4)     |
| ..=      | Range from start to end (inclusive) | 1..=5 (1, 2, 3, 4, 5) |

### The ? Operator

The `?` operator is used for error propagation in Rust. It is used to quickly return an error if a `Result` or `Option` is an `Err` or `None`.

```rs
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err("Division by zero".into())
    } else {
        Ok(x / y)
    }
}

fn main() -> Result<(), String> {
    let result = divide(5, 0)?; // Will return early with error if division by zero
    println!("{}", result);
    Ok(())
}
```
