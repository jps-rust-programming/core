# data types in Rust

Rust is a `statically-typed` language, meaning that the type of every variable is known at compile time. Rust offers a rich set of primitive and composite data types, which are categorized into `1. scalar types` and `2. compound types`.

## 1. Scalar Types

Scalar types represent a single value. Rust has four basic scalar types:

- integers
- floating-point numbers,
- booleans, and
- characters.

## 2. Compound Types

Compound types allow you to combine multiple values into one type. Rust has two kinds of compound types:

1. tuples and
2. arrays.

## 3. String Types

Rust provides two types for handling text:

- `String`: A growable, `heap-allocated` string type. It can store text data and be modified.
- `&str`: A string slice, which is a reference to a part of a string, typically a string literal or a slice of a String.

## 4. Other Special Types

Rust also has some other special types that serve particular purposes:

- **Unit Type (())**
- **Option Type (Option<T>)**

  `Option<T>` is a powerful enum that is used to represent an optional value. It can either be `Some(T)` where T is a value, or None indicating the absence of a value.

  - `Some(T)` represents a value of type T.
  - `None` represents no value.

- **Result Type (Result<T, E>)**

  `Result<T, E>` is used for error handling and represents either a success `(Ok(T))` or an error `(Err(E))`.

## 5. Type Aliases

Rust allows you to define type aliases, which are used to give a new name to an existing type. Type aliases are often used to simplify complex types or to give a meaningful name to a type.

```rs
type Kilometers = i32;

let distance: Kilometers = 100;
```

## 6. Pointers and References

Rust has a unique system for working with memory, and it uses references and pointers to enable memory safety without needing a garbage collector.

- `Reference (&T)`: A reference to a value of type T. It is used to borrow data without taking ownership.
- `Mutable Reference (&mut T)`: A mutable reference to a value, which allows you to modify the value.

## 7. Type Inference

Rust has powerful type inference, which means that the compiler can often determine the type of a variable without you needing to explicitly specify it.

```rs
let a = 5; // inferred as i32
let b = 3.14; // inferred as f64
```
