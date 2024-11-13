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
