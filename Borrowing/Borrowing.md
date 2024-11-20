# Borrowing

> How do iterators use borrowing to efficiently process collections?

## Iterators and Borrowing in Rust

Iterators in Rust are a powerful tool for processing collections efficiently. They allow you to iterate over elements without consuming `ownership` of the underlying collection. This is achieved through `borrowing`.

**How Iterators Use Borrowing:**

### 1. Borrowing Elements:

When you call the `iter()` method on a collection, it returns an iterator. This iterator borrows elements from the collection, allowing you to access them without taking `ownership`.

```rust
let numbers = vec![1, 2, 3];
for number in numbers.iter() {
    println!("{}", number);
}
```

### 2. Lazy Evaluation:

Many iterator methods, like `map`, `filter`, and `fold`, are lazy. This means they don't process the entire collection at once. Instead, they process elements as needed, which can be more efficient, especially for large datasets. This lazy evaluation is often implemented using iterators, which borrow elements from the underlying collection as needed.

### 3. Avoiding Unnecessary Copying:

By borrowing elements, iterators avoid unnecessary copying of data. This is crucial for performance, especially when dealing with large collections.
