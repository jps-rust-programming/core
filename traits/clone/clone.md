# Debug Trait Bound

If the generic type T doesn't implement the `Debug` trait, attempting to print it using `println!("{:?}", value)` will result in a compilation error because Debug is required for formatted output like `{:?}`.

To handle this, we can use the `Debug` trait bound alongside the `Clone` trait bound. This ensures that T can be cloned and also printed with `{:?}` formatting.
