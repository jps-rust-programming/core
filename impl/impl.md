# impl keyword

The `impl keyword` is central to Rust’s `ownership`, `borrowing`, and `type system`. Understanding it fully will greatly enhance your ability to write clean, idiomatic, and efficient Rust code.

## Associated Functions in impl Blocks

Associated functions are those that `do not take self as their first parameter`. These are usually used for `constructors` or functions that are associated with the type but `don't operate on any instance`.

> An instance method is a method that operates on a specific instance of a type, usually with the first parameter being `self`, `&self`, or `&mut self`.

- &self allows the method to read from the instance.
- &mut self allows the method to mutate the instance.
