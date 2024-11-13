# traits, modules, and crates in Rust

## 1. Traits in Rust

A trait is a way to define shared behavior for types. It’s similar to `interfaces` in languages like Java or TypeScript but with more flexibility. `Traits are used to specify methods that types` (e.g., `structs`, `enums`, etc.) must implement.

> Traits are commonly used for defining shared functionality across different types, `enabling polymorphism`.

### Defining and Implementing Traits

You define a trait with the trait keyword and implement it for a type using the `impl` keyword.

**Example:**

```rs
// Defining a trait called `Greet`
trait Greet {
    fn greet(&self) -> String;
}

// Implementing the `Greet` trait for the `Person` struct
struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

fn main() {
    let person = Person { name: String::from("Alice") };
    println!("{}", person.greet());  // "Hello, Alice!"
}
```

- `Trait definition`: The trait Greet defines a method greet that types can implement.
- `Implementation`: `impl` Greet for Person means that Person is implementing the greet method, adhering to the Greet trait.

## 2. Modules in Rust

A module in Rust is a way to organize and group related code. Modules can contain functions, structs, traits, enums, constants, and other modules. They help manage larger codebases by creating `namespaces` and controlling visibility of code.

- Modules are defined with the `mod` keyword. By default, the items within a module are `private`, and you need the `pub` keyword to expose them.
