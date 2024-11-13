# Ownership and Borrowing

### Key Concepts:

- `Ownership`: A variable owns the value it holds. When a variable goes out of scope, the memory is freed.
- `Borrowing`: Rust allows references to values without transferring ownership, either as immutable or mutable references.
- `Move Semantics`: When you assign a value to another variable, ownership is transferred, and the original variable can no longer be used.
- `Copy`: Types that implement the Copy trait can be duplicated trivially, meaning they can be copied instead of moved.

### Example Questions:

- What happens when you try to use a variable after its ownership has been moved?
- Explain the difference between borrowing immutably (&T) and borrowing mutably (&mut T).
- How does Rust ensure memory safety without a garbage collector?
