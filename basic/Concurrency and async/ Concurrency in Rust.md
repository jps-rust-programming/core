# Concurrency in Rust

## Key Concepts:

- `Threads`: Rust provides the std::thread module for spawning threads.
- `Mutexes and Arc`: Mutex<T> is used for mutual exclusion, and Arc<T> allows safe sharing of data between threads.
- `Send and Sync`: Types that are safe to send across threads (Send) and those that are safe to share across threads (Sync).

### Example Questions:

1. How does Rust prevent data races at compile time?
2. What is the difference between Send and Sync in Rust?
3. Can you explain how Arc and Mutex work together in Rust?
