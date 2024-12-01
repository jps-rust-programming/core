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

### Primitive Data Types

Rust provides a set of primitive data types:

#### Integer Types:

- Signed integers: i8, i16, i32, i64, isize
- Unsigned integers: u8, u16, u32, u64, usize
- isize and usize are architecture-dependent, typically 32-bit on 32-bit systems and 64-bit on 64-bit systems.

#### Floating-Point Types:

- f32: 32-bit single-precision float
- f64: 64-bit double-precision float

#### Boolean Type:

- bool: Represents a boolean value, either true or false.
  Character Type:

- char: Represents a single Unicode character, enclosed in single quotes.

### Compound Data Types

**- Tuple**:

A fixed-size collection of elements of different types.
Example: let tup: (i32, f64, u8) = (500, 6.4, 1);

**- Array**:

A fixed-size collection of elements of the same type.
Example: let a: [i32; 5] = [1, 2, 3, 4, 5];

**- Slice**:

A reference to a contiguous sequence of elements in an array.
Example: let slice = &a[1..3];

**- String**:

A growable sequence of characters.
Example: let s = String::from("hello");

### Ownership and Borrowing

Rust's ownership system ensures memory safety and prevents data races. Key concepts include:

- `Ownership`: Each value in Rust has an owner.
- `Borrowing`: You can borrow a value to use it without taking ownership.
- `Lifetime Annotations`: Specify the lifetime of a borrow.

```rust
fn main() {
    let x = 5; // Integer
    let y = 2.5; // Float
    let z = true; // Boolean
    let c = 'A'; // Character

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let s = String::from("hello");

    println!("Integer: {}", x);
    println!("Float: {}", y);
    println!("Boolean: {}", z);
    println!("Character: {}", c);
    println!("Tuple: {:?}", tup);
    println!("Array: {:?}", a);
    println!("String: {}", s);
}
```
