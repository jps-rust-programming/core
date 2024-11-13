# Naming conventions in rust

## 1. Snake Case for Variable and Function Names

Rust uses `snake_case` for variables, function names, and other identifiers that are not types or constants. This means all letters are `lowercase`, and words are separated by underscores `(_)`.

**Examples:**

```rs

let my_variable = 42;
fn calculate_total() -> i32 {
    // function body
}
```

## 2. Camel Case for Structs, Enums, and Traits

For structs, enums, and traits, Rust uses CamelCase (also known as PascalCase), where each word is capitalized, and no underscores are used.

**Examples:**

```rs
struct MyStruct {
    field1: i32,
    field2: String,
}

enum Direction {
    North,
    South,
    East,
    West,
}

trait Drawable {
    fn draw(&self);
}
```

## 3. Uppercase with Underscores for Constants and Static Variables

Constants and static variables (including const values) are written in `UPPERCASE_WITH_UNDERSCORES`, which distinguishes them from regular variables and functions.

**Examples:**

```rs
const MAX_RETRIES: u32 = 5;
static GLOBAL_VARIABLE: i32 = 100;
```

## 4. Modules and Files

For module names, Rust uses `snake_case`. The names of files that contain modules also follow the same convention. A module named utils would correspond to a file named utils.rs.

**Example:**

```rs
mod my_module; // refers to `my_module.rs` or `my_module/mod.rs`
```

## 5. Type Aliases

Type aliases follow the `CamelCase` convention, similar to `structs`, `enums`, and `traits`.

**Example:**

```rs
type MyAlias = Vec<i32>;
```

## 6. Enum Variants

Enum variants are named using CamelCase, starting with an uppercase `letter`, similar to `struct` and `trait` names.

**Example:**

```rs
enum Color {
    Red,
    Green,
    Blue,
}
```

## 7. Private and Public Items

Rust uses snake_case for private members and CamelCase for public structs, enums, and traits. The convention for visibility follows the item’s case style:

- Private fields in structs or enums: `snake_case`

- Public fields in structs or enums: `CamelCase` (though it’s recommended to be explicit about visibility for clarity).

**Example:**

```rs
pub struct User {
    pub name: String,
    age: u32,  // private field, snake_case is fine
}
```

## 8. Crate Names

Crate names are generally in `snake_case`, and this convention extends to the names of packages when publishing to crates.io.

**Example:**

```rs
[package]
name = "my_crate_name"
```

## 9. Avoid Hungarian Notation

Rust does not use Hungarian notation (prefixing variable names with their types), which is common in other languages like `C or C++`. For example, avoid naming variables like strName or i32Counter. Instead, use descriptive names in snake_case.

## 10. Prefix for Traits and Types in Some Cases

Sometimes, the name of a trait can reflect the behavior it represents. In certain cases, traits use a common suffix like `-able` for traits that represent capabilities or actions:

```rs
trait Cloneable {
    fn clone(&self) -> Self;
}
```
