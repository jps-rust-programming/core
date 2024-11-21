**Example: A Simple Library for Logging Events**

Imagine you're building a simple logging system for an application that can log different types of events. You want to be able to print `debug` information about the events in a standardized way. Some events might be simple types (e.g., `integers`, `strings`), while others could be more complex structs. To ensure all events can be printed using `println!("{:?}", ...)`, you need to make sure they implement the `Debug` trait.

We'll create a logging system where you can log events of any type that implements `Debug`.

**Step 1: Define the Event Struct**

Let's define a struct called Event that can hold information about an event. The event could be of any type, but for this example, we will have different types of events that can be logged.

```rust
use std::fmt;

// Define the Event struct with a name and description
struct Event<T> {
    name: String,
    description: String,
    data: T,
}

// Implement Debug for the Event struct
impl<T: fmt::Debug> fmt::Debug for Event<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Event {{ name: {}, description: {}, data: {:?} }}", self.name, self.description, self.data)
    }
}
```

**In the Event struct:**

- We use generics (T) to allow Event to hold any type of data.
- We implement the Debug trait for Event only if `T` (the type of data) also implements `Debug`. This is done using the bound `T: fmt::Debug` in the `impl` block.

**Step 2: Create Different Types of Events**

Next, we'll create some events of different types. One event might have a simple `String` as its data, and another might have more complex data like a `custom struct`.

```rust
#[derive(Debug)]
struct User {
    id: u32,
    username: String,
}

fn main() {
    // Simple event with a string as data
    let event1 = Event {
        name: "User Login".to_string(),
        description: "User logs into the system".to_string(),
        data: "Login successful",
    };

    // Event with a custom struct as data
    let user = User {
        id: 1,
        username: "alice".to_string(),
    };
    let event2 = Event {
        name: "User Created".to_string(),
        description: "A new user account is created".to_string(),
        data: user,
    };

    // Print events
    println!("{:?}", event1);  // Logs: Event { name: User Login, description: User logs into the system, data: "Login successful" }
    println!("{:?}", event2);  // Logs: Event { name: User Created, description: A new user account is created, data: User { id: 1, username: "alice" } }
}
```

**Explanation:**

- `User Struct`: We derive Debug for the User struct so that its id and username fields can be printed with `println!("{:?}", ...)`.
- `Event Struct`: The Event struct is generic, and we've ensured that it can only be debugged if its data type `(T)` implements `Debug`. In our example, the User struct is debugged successfully because User implements Debug via `#[derive(Debug)]`.
- `Logging Events`: We create two events — one with a String as the event data and another with a User struct as the event data. Both types are printable using `println!("{:?}", ...)` because they implement `Debug`.

**Step 3: Use a Generic Function with Debug Bound**

Now let's create a function that can `log` any `event`, but only if the event's data can be debugged.

```rust
fn log_event<T: fmt::Debug>(event: Event<T>) {
    println!("{:?}", event);
}
```
