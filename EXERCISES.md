# Rust Learning Exercises

A comprehensive set of exercises to practice Rust concepts. Each exercise includes hints and references to the relevant crate in this repository.

## How to Use This Guide

1. **Read the chapter examples first** - Each crate contains commented examples
2. **Try the exercise** - Attempt without looking at hints
3. **Use hints progressively** - Only reveal hints when stuck
4. **Check your solution** - Run `cargo test` or `cargo run` to verify

---

## Chapter 3-4: Variables, Types, and Functions

### Exercise 1: Temperature Converter
**Create a function that converts Fahrenheit to Celsius and vice versa.**

```rust
// TODO: Implement these functions
fn fahrenheit_to_celsius(f: f64) -> f64 {
    // Your code here
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    // Your code here
}
```

<details>
<summary>Hint 1</summary>
The formula is: C = (F - 32) × 5/9
</details>

<details>
<summary>Hint 2</summary>
Remember to use floating point division (5.0/9.0), not integer division
</details>

<details>
<summary>Solution</summary>

```rust
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
```
</details>

---

### Exercise 2: Fibonacci Generator
**Generate the nth Fibonacci number.**

```rust
fn fibonacci(n: u32) -> u64 {
    // Your code here
}
```

<details>
<summary>Hint 1</summary>
Use a loop with two variables to track the previous two numbers
</details>

<details>
<summary>Hint 2</summary>
Base cases: fib(0) = 0, fib(1) = 1
</details>

<details>
<summary>Solution</summary>

```rust
fn fibonacci(n: u32) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    let mut prev = 0u64;
    let mut curr = 1u64;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
```
</details>

---

## Chapter 5: Ownership (See: `ch04-ownership/`)

### Exercise 3: String Ownership
**Fix this code without using clone():**

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);  // ERROR: s1 was moved
}
```

<details>
<summary>Hint 1</summary>
Use a reference (&) to borrow instead of move
</details>

<details>
<summary>Hint 2</summary>
Consider which variable actually needs ownership of the String
</details>

<details>
<summary>Solution</summary>

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;  // Borrow instead of move
    println!("{}, world!", s1);
    println!("s2 = {}", s2);
}
```
</details>

---

### Exercise 4: Mutable References
**Complete this function that appends a greeting to a string:**

```rust
fn add_greeting(s: /* ??? */) {
    // Add ", welcome!" to the string
}

fn main() {
    let mut message = String::from("Hello");
    add_greeting(/* ??? */);
    println!("{}", message);  // Should print: Hello, welcome!
}
```

<details>
<summary>Hint 1</summary>
You need a mutable reference: &mut String
</details>

<details>
<summary>Hint 2</summary>
Use push_str() to append to a String
</details>

<details>
<summary>Solution</summary>

```rust
fn add_greeting(s: &mut String) {
    s.push_str(", welcome!");
}

fn main() {
    let mut message = String::from("Hello");
    add_greeting(&mut message);
    println!("{}", message);
}
```
</details>

---

## Chapter 6: Structs (See: `ch05-structs/`)

### Exercise 5: Rectangle Area
**Create a Rectangle struct with methods for area and perimeter:**

```rust
struct Rectangle {
    // Define fields
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        // Constructor
    }

    fn area(&self) -> u32 {
        // Calculate area
    }

    fn perimeter(&self) -> u32 {
        // Calculate perimeter
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        // Returns true if self can completely contain other
    }
}
```

<details>
<summary>Hint 1</summary>
Use `self.width` and `self.height` in methods
</details>

<details>
<summary>Hint 2</summary>
For can_hold: self.width > other.width AND self.height > other.height
</details>

<details>
<summary>Solution</summary>

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
</details>

---

## Chapter 7: Enums and Pattern Matching (See: `ch06-enums/`)

### Exercise 6: Traffic Light
**Create an enum for traffic lights with a method to get duration:**

```rust
enum TrafficLight {
    // Define variants
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        // Return duration in seconds
        // Red: 60, Yellow: 10, Green: 45
    }
}
```

<details>
<summary>Hint 1</summary>
Use match on self to return different values per variant
</details>

<details>
<summary>Solution</summary>

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 45,
        }
    }
}
```
</details>

---

### Exercise 7: Option Handling
**Complete this function that safely divides two numbers:**

```rust
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    // Return None if b is 0, otherwise return Some(a/b)
}

fn main() {
    match safe_divide(10, 2) {
        // Handle the result
    }
}
```

<details>
<summary>Hint 1</summary>
Check if b == 0 first, return None in that case
</details>

<details>
<summary>Solution</summary>

```rust
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match safe_divide(10, 2) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero!"),
    }
}
```
</details>

---

## Chapter 8: Collections (See: `ch08-common-collections/`)

### Exercise 8: Word Counter
**Count word frequencies in a string using HashMap:**

```rust
use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, u32> {
    // Return a map of word -> count
}
```

<details>
<summary>Hint 1</summary>
Use text.split_whitespace() to get words
</details>

<details>
<summary>Hint 2</summary>
Use entry().or_insert(0) pattern to increment counts
</details>

<details>
<summary>Solution</summary>

```rust
use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    counts
}
```
</details>

---

### Exercise 9: Vector Statistics
**Calculate mean, median, and mode of a vector:**

```rust
fn statistics(numbers: &Vec<i32>) -> (f64, f64, i32) {
    // Return (mean, median, mode)
}
```

<details>
<summary>Hint 1</summary>
Mean = sum / count. Use iter().sum() and len()
</details>

<details>
<summary>Hint 2</summary>
For median, sort a copy and get middle element(s)
</details>

<details>
<summary>Hint 3</summary>
For mode, use HashMap to count occurrences, then find max
</details>

<details>
<summary>Solution</summary>

```rust
use std::collections::HashMap;

fn statistics(numbers: &Vec<i32>) -> (f64, f64, i32) {
    // Mean
    let sum: i32 = numbers.iter().sum();
    let mean = sum as f64 / numbers.len() as f64;

    // Median
    let mut sorted = numbers.clone();
    sorted.sort();
    let mid = sorted.len() / 2;
    let median = if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) as f64 / 2.0
    } else {
        sorted[mid] as f64
    };

    // Mode
    let mut counts = HashMap::new();
    for &n in numbers {
        *counts.entry(n).or_insert(0) += 1;
    }
    let mode = *counts.iter().max_by_key(|&(_, count)| count).unwrap().0;

    (mean, median, mode)
}
```
</details>

---

## Chapter 9: Error Handling

### Exercise 10: File Reader with Errors
**Create a function that reads a file and returns Result:**

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    // Read file contents and return Result
}
```

<details>
<summary>Hint 1</summary>
Use File::open(path)? to propagate errors with ?
</details>

<details>
<summary>Hint 2</summary>
Create a String, use file.read_to_string(&mut content)?
</details>

<details>
<summary>Solution</summary>

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```
</details>

---

## Chapter 10: Generics (See: `ch10-generics/`)

### Exercise 11: Generic Pair
**Create a generic Pair struct that can hold two values of any type:**

```rust
struct Pair<T, U> {
    // Define fields
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        // Constructor
    }

    fn swap(self) -> Pair<U, T> {
        // Return a new Pair with values swapped
    }
}
```

<details>
<summary>Hint 1</summary>
Use T and U as type parameters for the two values
</details>

<details>
<summary>Solution</summary>

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}
```
</details>

---

## Chapter 10: Traits (See: `ch10-traits/`)

### Exercise 12: Printable Trait
**Create a trait that provides a custom print format:**

```rust
trait Printable {
    fn format(&self) -> String;

    fn print(&self) {
        println!("{}", self.format());
    }
}

// Implement for these structs:
struct Person { name: String, age: u32 }
struct Product { name: String, price: f64 }
```

<details>
<summary>Hint 1</summary>
Use format!() macro to create formatted strings
</details>

<details>
<summary>Solution</summary>

```rust
trait Printable {
    fn format(&self) -> String;

    fn print(&self) {
        println!("{}", self.format());
    }
}

struct Person { name: String, age: u32 }

impl Printable for Person {
    fn format(&self) -> String {
        format!("Person: {} ({} years old)", self.name, self.age)
    }
}

struct Product { name: String, price: f64 }

impl Printable for Product {
    fn format(&self) -> String {
        format!("Product: {} - ${:.2}", self.name, self.price)
    }
}
```
</details>

---

## Chapter 10: Lifetimes (See: `ch10-lifetimes/`)

### Exercise 13: Longest String
**Fix the lifetime annotations:**

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

<details>
<summary>Hint 1</summary>
The return type needs a lifetime that relates to both inputs
</details>

<details>
<summary>Solution</summary>

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
</details>

---

## Chapter 13: Closures (See: `ch13-closures/`)

### Exercise 14: Closure Counter
**Create a closure that counts how many times it's been called:**

```rust
fn make_counter() -> impl FnMut() -> u32 {
    // Return a closure that increments and returns a counter
}
```

<details>
<summary>Hint 1</summary>
Create a mutable variable before the closure, move it in
</details>

<details>
<summary>Hint 2</summary>
Use `move` to take ownership of the counter
</details>

<details>
<summary>Solution</summary>

```rust
fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

fn main() {
    let mut counter = make_counter();
    println!("{}", counter());  // 1
    println!("{}", counter());  // 2
    println!("{}", counter());  // 3
}
```
</details>

---

## Chapter 13: Iterators (See: `ch13-iterators/`)

### Exercise 15: Iterator Chain
**Use iterator methods to transform a list:**

```rust
// Given: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// Return: Sum of squares of even numbers
fn sum_of_even_squares(numbers: Vec<i32>) -> i32 {
    // Use filter, map, and sum
}
```

<details>
<summary>Hint 1</summary>
Use .iter().filter(|x| x % 2 == 0).map(|x| x * x).sum()
</details>

<details>
<summary>Solution</summary>

```rust
fn sum_of_even_squares(numbers: Vec<i32>) -> i32 {
    numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}
```
</details>

---

## Chapter 15: Smart Pointers (See: `ch15-smart-pointers/`)

### Exercise 16: Linked List with Box
**Implement a simple linked list using Box:**

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        // Create empty list
    }

    fn prepend(self, value: i32) -> List {
        // Add value to front
    }

    fn len(&self) -> usize {
        // Count elements
    }
}
```

<details>
<summary>Hint 1</summary>
Nil represents an empty list
</details>

<details>
<summary>Hint 2</summary>
For len(), use match and recursion
</details>

<details>
<summary>Solution</summary>

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, value: i32) -> List {
        Cons(value, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            Nil => 0,
            Cons(_, tail) => 1 + tail.len(),
        }
    }
}
```
</details>

---

## Chapter 16: Concurrency (See: `ch16-concurrency/`)

### Exercise 17: Parallel Sum
**Sum a large vector using multiple threads:**

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn parallel_sum(numbers: Vec<i32>, num_threads: usize) -> i32 {
    // Split work across threads and sum results
}
```

<details>
<summary>Hint 1</summary>
Use Arc<Mutex<i32>> to share the sum across threads
</details>

<details>
<summary>Hint 2</summary>
Split the vector into chunks with chunks()
</details>

<details>
<summary>Solution</summary>

```rust
use std::thread;
use std::sync::{Arc, Mutex};

fn parallel_sum(numbers: Vec<i32>, num_threads: usize) -> i32 {
    let sum = Arc::new(Mutex::new(0));
    let chunk_size = (numbers.len() + num_threads - 1) / num_threads;
    let numbers = Arc::new(numbers);

    let mut handles = vec![];

    for i in 0..num_threads {
        let sum = Arc::clone(&sum);
        let numbers = Arc::clone(&numbers);

        let handle = thread::spawn(move || {
            let start = i * chunk_size;
            let end = std::cmp::min(start + chunk_size, numbers.len());

            let partial: i32 = numbers[start..end].iter().sum();
            *sum.lock().unwrap() += partial;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *sum.lock().unwrap()
}
```
</details>

---

## Chapter 17: Async (See: `ch17-async-await/`)

### Exercise 18: Async Timeout
**Create an async function that returns a value or times out:**

```rust
// Pseudo-code (requires async runtime like tokio)
async fn with_timeout<T>(future: impl Future<Output = T>, timeout_ms: u64) -> Option<T> {
    // Return Some(result) if future completes before timeout
    // Return None if timeout occurs first
}
```

<details>
<summary>Hint 1</summary>
Use tokio::select! or trpl::race to race the future against a sleep
</details>

<details>
<summary>Solution (using trpl)</summary>

```rust
use trpl::Either;
use std::time::Duration;

async fn with_timeout<T>(
    future: impl std::future::Future<Output = T>,
    timeout_ms: u64
) -> Option<T> {
    let timeout = async {
        trpl::sleep(Duration::from_millis(timeout_ms)).await;
    };

    match trpl::race(future, timeout).await {
        Either::Left(result) => Some(result),
        Either::Right(()) => None,
    }
}
```
</details>

---

## Chapter 18: OOP Patterns (See: `ch18-oop-patterns/`)

### Exercise 19: Builder Pattern
**Implement a builder pattern for a complex struct:**

```rust
struct Email {
    to: String,
    from: String,
    subject: String,
    body: String,
}

struct EmailBuilder {
    // Fields
}

impl EmailBuilder {
    fn new() -> Self { /* ... */ }
    fn to(self, to: &str) -> Self { /* ... */ }
    fn from(self, from: &str) -> Self { /* ... */ }
    fn subject(self, subject: &str) -> Self { /* ... */ }
    fn body(self, body: &str) -> Self { /* ... */ }
    fn build(self) -> Result<Email, &'static str> { /* ... */ }
}
```

<details>
<summary>Hint 1</summary>
Use Option<String> for optional fields in the builder
</details>

<details>
<summary>Solution</summary>

```rust
struct Email {
    to: String,
    from: String,
    subject: String,
    body: String,
}

struct EmailBuilder {
    to: Option<String>,
    from: Option<String>,
    subject: Option<String>,
    body: Option<String>,
}

impl EmailBuilder {
    fn new() -> Self {
        EmailBuilder {
            to: None,
            from: None,
            subject: None,
            body: None,
        }
    }

    fn to(mut self, to: &str) -> Self {
        self.to = Some(to.to_string());
        self
    }

    fn from(mut self, from: &str) -> Self {
        self.from = Some(from.to_string());
        self
    }

    fn subject(mut self, subject: &str) -> Self {
        self.subject = Some(subject.to_string());
        self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    fn build(self) -> Result<Email, &'static str> {
        Ok(Email {
            to: self.to.ok_or("Missing 'to' field")?,
            from: self.from.ok_or("Missing 'from' field")?,
            subject: self.subject.unwrap_or_default(),
            body: self.body.unwrap_or_default(),
        })
    }
}
```
</details>

---

## Chapter 19: Patterns (See: `ch19-pattern-matching/`)

### Exercise 20: Pattern Matching Challenge
**Use all pattern features to parse commands:**

```rust
enum Command {
    Move { x: i32, y: i32 },
    Say(String),
    Quit,
    ChangeColor(u8, u8, u8),
}

fn execute(cmd: Command) {
    // Use match with:
    // - Destructuring
    // - Guards
    // - @ bindings
    // to handle all commands
}
```

<details>
<summary>Hint 1</summary>
Use match with guards like `if x > 0`
</details>

<details>
<summary>Solution</summary>

```rust
enum Command {
    Move { x: i32, y: i32 },
    Say(String),
    Quit,
    ChangeColor(u8, u8, u8),
}

fn execute(cmd: Command) {
    match cmd {
        Command::Quit => println!("Goodbye!"),

        Command::Move { x: 0, y: 0 } => println!("Staying put"),
        Command::Move { x, y } if x == y => println!("Moving diagonally to ({}, {})", x, y),
        Command::Move { x, y } => println!("Moving to ({}, {})", x, y),

        Command::Say(msg) if msg.is_empty() => println!("(silence)"),
        Command::Say(msg) => println!("Saying: {}", msg),

        Command::ChangeColor(r @ 200..=255, g, b) => {
            println!("Bright red detected! RGB({}, {}, {})", r, g, b)
        }
        Command::ChangeColor(r, g, b) => {
            println!("Changing color to RGB({}, {}, {})", r, g, b)
        }
    }
}
```
</details>

---

## 🎯 Challenge Projects

### Project 1: Mini Database
Create an in-memory key-value store with:
- CRUD operations
- Persistence to file (JSON)
- Command-line interface
- Error handling with Result

### Project 2: Async Web Scraper
Build a concurrent web scraper that:
- Fetches multiple URLs in parallel
- Extracts specific data (titles, links)
- Handles errors gracefully
- Uses rate limiting

### Project 3: Chat Server
Implement a simple chat server with:
- Multiple clients using threads
- Message broadcasting using channels
- User nicknames
- Graceful shutdown

---

## Progress Tracker

| Chapter | Topic | Exercises | Completed |
|---------|-------|-----------|-----------|
| 3-4 | Basics | 1-2 | ☐ |
| 5 | Ownership | 3-4 | ☐ |
| 6 | Structs | 5 | ☐ |
| 7 | Enums | 6-7 | ☐ |
| 8 | Collections | 8-9 | ☐ |
| 9 | Errors | 10 | ☐ |
| 10 | Generics | 11 | ☐ |
| 10 | Traits | 12 | ☐ |
| 10 | Lifetimes | 13 | ☐ |
| 13 | Closures | 14 | ☐ |
| 13 | Iterators | 15 | ☐ |
| 15 | Smart Ptrs | 16 | ☐ |
| 16 | Concurrency | 17 | ☐ |
| 17 | Async | 18 | ☐ |
| 18 | OOP | 19 | ☐ |
| 19 | Patterns | 20 | ☐ |

---

## Study Path Recommendation

### Week 1-2: Fundamentals
1. Read `src/ownership.rs`, `src/structs.rs`, `src/enums.rs`
2. Complete Exercises 1-7
3. Run examples with `cargo run`

### Week 3: Collections & Errors
1. Study `common-collections/`
2. Complete Exercises 8-10
3. Practice with your own examples

### Week 4: Generics, Traits, Lifetimes
1. Read `generics/`, `traits/`, `lifetimes/`
2. Complete Exercises 11-13
3. These are crucial - take your time!

### Week 5: Functional Features
1. Study `closures/`, `iterators/`
2. Complete Exercises 14-15
3. Practice chaining iterator methods

### Week 6: Memory & Concurrency
1. Read `smart_pointer/`, `consurrency/`
2. Complete Exercises 16-17
3. Understand when to use each smart pointer

### Week 7-8: Advanced Topics
1. Study `hello-async/`, `oop-patterns/`, `pattern-matching/`, `advanced-features/`
2. Complete Exercises 18-20
3. Try the challenge projects

---

Happy Rusting! 🦀
