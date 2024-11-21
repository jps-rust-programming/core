# Debug Trait Bound

If the generic type T doesn't implement the `Debug` trait, attempting to print it using `println!("{:?}", value)` will result in a compilation error because Debug is required for formatted output like `{:?}`.

To handle this, we can use the `Debug` trait bound alongside the `Clone` trait bound. This ensures that T can be cloned and also printed with `{:?}` formatting.

## Adding Debug Trait Bound

We can modify the generic struct `Container<T>` to require both `Clone` and `Debug` bounds on T. This way, we ensure that:

- T can be cloned (using Clone).
- T can be printed in debug format (using Debug).

### Updated Example with Both Clone and Debug:

```rust
// Define a struct that holds a vector of elements of type T
#[derive(Debug)]
struct Container<T> {
    items: Vec<T>,
}

impl<T: Clone + std::fmt::Debug> Container<T> {
    // Constructor to create a new container
    fn new(items: Vec<T>) -> Self {
        Container { items }
    }

    // Method to clone elements of the container
    fn clone_elements(&self) -> Vec<T> {
        self.items.clone() // `clone()` is available because T implements `Clone`
    }

    // Print the contents of the container
    fn display(&self) {
        println!("{:?}", self.items);  // T must implement `Debug` to use `{:?}`
    }
}

fn main() {
    // Create a container with integers (which implement Clone and Debug)
    let int_container = Container::new(vec![1, 2, 3, 4, 5]);

    // Display the original container
    println!("Original container:");
    int_container.display();

    // Clone the elements of the container
    let cloned_ints = int_container.clone_elements();
    println!("Cloned elements:");
    println!("{:?}", cloned_ints);

    // Create a container with strings (which implement Clone and Debug)
    let string_container = Container::new(vec!["Hello".to_string(), "World".to_string()]);

    // Display the original string container
    println!("\nOriginal string container:");
    string_container.display();

    // Clone the elements of the string container
    let cloned_strings = string_container.clone_elements();
    println!("Cloned elements:");
    println!("{:?}", cloned_strings);
}
```

### Explanation of Changes:

#### Trait Bounds:

- `impl<T: Clone + std::fmt::Debug>`: The Container<T> implementation now requires T to implement both Clone and Debug.
- The `Clone` bound is necessary because we need to clone the elements of the container.
- The `Debug` bound is necessary because we need to use `println!("{:?}", self.items)` to print the contents of the container in a debug format.

#### Using Debug in display:

- The display method now prints the contents of `self.items` using the debug format `{:?}`, which requires T to implement the `Debug` trait.
