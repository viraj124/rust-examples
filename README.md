# Rust Learning Examples

A comprehensive collection of Rust code examples organized by chapter, following "The Rust Programming Language" book.

## Getting Started

```bash
# Clone the repository
git clone <repo-url>
cd rust_examples

# Build all crates
cargo build

# Run a specific chapter
cargo run -p ch02-guessing-game

# Run tests for a chapter
cargo test -p ch11-testing
```

## Chapter Index

| Chapter | Crate | Topics |
|---------|-------|--------|
| **Ch 2** | [ch02-guessing-game](ch02-guessing-game/) | First program, I/O, random numbers, match |
| **Ch 3** | [ch03-common-concepts](ch03-common-concepts/) | Variables, data types, functions, control flow |
| **Ch 4** | [ch04-ownership](ch04-ownership/) | Ownership, borrowing, references, slices |
| **Ch 5** | [ch05-structs](ch05-structs/) | Structs, methods, associated functions |
| **Ch 6** | [ch06-enums](ch06-enums/) | Enums, Option, match, if let |
| **Ch 7** | [ch07-modules-packages](ch07-modules-packages/) | Modules, packages, crates, pub/use |
| **Ch 8** | [ch08-common-collections](ch08-common-collections/) | Vec, String, HashMap |
| **Ch 10** | [ch10-generics](ch10-generics/) | Generic types and functions |
| **Ch 10** | [ch10-traits](ch10-traits/) | Traits, trait bounds, impl Trait |
| **Ch 10** | [ch10-lifetimes](ch10-lifetimes/) | Lifetime annotations, elision rules |
| **Ch 11** | [ch11-testing](ch11-testing/) | Unit tests, integration tests, test attributes |
| **Ch 12** | [ch12-minigrep-project](ch12-minigrep-project/) | CLI app, file I/O, error handling |
| **Ch 13** | [ch13-closures](ch13-closures/) | Closures, Fn traits, capturing |
| **Ch 13** | [ch13-iterators](ch13-iterators/) | Iterators, adapters, consumers |
| **Ch 15** | [ch15-smart-pointers](ch15-smart-pointers/) | Box, Rc, RefCell, Deref, Drop |
| **Ch 15** | [ch15-reference-cycles](ch15-reference-cycles/) | Weak references, preventing cycles |
| **Ch 16** | [ch16-concurrency](ch16-concurrency/) | Threads, channels, Mutex, Arc |
| **Ch 17** | [ch17-async-await](ch17-async-await/) | Async/await, Futures, Streams, Pin |
| **Ch 18** | [ch18-oop-patterns](ch18-oop-patterns/) | Trait objects, State pattern, Strategy pattern |
| **Ch 19** | [ch19-pattern-matching](ch19-pattern-matching/) | Pattern syntax, destructuring, guards |
| **Ch 20** | [ch20-advanced-features](ch20-advanced-features/) | Unsafe, advanced traits, macros |

## Exercises

See [EXERCISES.md](EXERCISES.md) for hands-on practice exercises with progressive hints and solutions.

## Project Structure

```
rust_examples/
├── Cargo.toml              # Workspace configuration
├── README.md               # This file
├── EXERCISES.md            # Practice exercises
├── ch02-guessing-game/     # Chapter 2
├── ch03-common-concepts/   # Chapter 3
├── ch04-ownership/         # Chapter 4
├── ...                     # More chapters
└── ch20-advanced-features/ # Chapter 20
```

## Running Examples

Each chapter is a standalone crate that can be run independently:

```bash
# Run chapter 4 (ownership examples)
cargo run -p ch04-ownership

# Run chapter 17 (async examples)
cargo run -p ch17-async-await

# Run chapter 12 minigrep with arguments
cargo run -p ch12-minigrep-project -- poem.txt search_term
```

## Learning Path

**Beginner (Weeks 1-2):**
- Ch 2-6: Basic syntax, ownership, structs, enums

**Intermediate (Weeks 3-4):**
- Ch 7-13: Modules, collections, error handling, closures, iterators

**Advanced (Weeks 5-6):**
- Ch 15-20: Smart pointers, concurrency, async, patterns, advanced features

## Key Concepts Quick Reference

### Ownership Rules
1. Each value has exactly one owner
2. When owner goes out of scope, value is dropped
3. Ownership can be moved or borrowed

### Borrowing Rules
- Many immutable references (`&T`) OR one mutable reference (`&mut T`)
- References must always be valid

### Common Patterns
```rust
// Option handling
let x = some_option.unwrap_or(default);
let x = some_option?;  // propagate None

// Result handling
let x = some_result?;  // propagate Err
let x = some_result.unwrap_or_else(|e| handle_error(e));

// Iteration
for item in collection.iter() { }      // borrow
for item in collection.into_iter() { } // consume
for item in &mut collection { }        // mutable borrow
```

## Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Docs](https://doc.rust-lang.org/std/)
