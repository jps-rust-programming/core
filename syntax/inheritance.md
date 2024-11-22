# Key Points About Trait "Inheritance" in Rust:

- `Supertraits`: One trait can inherit functionality from another by declaring it as a `supertrait` (e.g., trait `Colorful`: `Shape`).
- `Trait bounds`: You can combine multiple traits together as bounds in functions and structs, ensuring that the type implements all the required traits.
- `Default methods`: Traits can provide default method implementations, and types can override these methods when necessary.
- `Trait methods`: A trait can define methods that must be implemented by types that implement the trait (unless they have a default).
