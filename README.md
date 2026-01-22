# Rust Learning Plan

A structured path from beginner to proficient Rust developer.

---

## Phase 1: Foundations (Week 1-2)

### 1.1 Setup & Hello World
- [ ] Install Rust via rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [ ] Verify installation: `rustc --version` and `cargo --version`
- [ ] Create first project: `cargo new hello_rust`
- [ ] Understand Cargo.toml and project structure

### 1.2 Basic Syntax
- [ ] Variables and mutability (`let`, `let mut`, `const`)
- [ ] Data types (integers, floats, bool, char, tuples, arrays)
- [ ] Functions and return values
- [ ] Control flow (`if`, `else`, `loop`, `while`, `for`)
- [ ] Comments and documentation

### 1.3 Exercises
- [ ] Create a temperature converter (Celsius/Fahrenheit)
- [ ] Generate nth Fibonacci number
- [ ] FizzBuzz implementation

---

## Phase 2: Ownership - Rust's Core Concept (Week 3-4)

### 2.1 Ownership Rules
- [ ] Each value has exactly one owner
- [ ] Value is dropped when owner goes out of scope
- [ ] Understand move semantics vs copy

### 2.2 References & Borrowing
- [ ] Immutable references (`&T`)
- [ ] Mutable references (`&mut T`)
- [ ] The borrowing rules (one mutable OR many immutable)
- [ ] Dangling references and how Rust prevents them

### 2.3 Slices
- [ ] String slices (`&str`)
- [ ] Array slices (`&[T]`)
- [ ] Why slices are safe

### 2.4 Exercises
- [ ] Write a function that takes ownership and returns it
- [ ] Implement a function using only references
- [ ] Create a word counter using string slices

---

## Phase 3: Structuring Data (Week 5-6)

### 3.1 Structs
- [ ] Defining and instantiating structs
- [ ] Tuple structs and unit structs
- [ ] Method syntax (`impl` blocks)
- [ ] Associated functions

### 3.2 Enums
- [ ] Defining enums with data
- [ ] The `Option<T>` enum (Rust's null alternative)
- [ ] The `Result<T, E>` enum

### 3.3 Pattern Matching
- [ ] `match` expressions (must be exhaustive)
- [ ] `if let` for single pattern matching
- [ ] `while let` for loops
- [ ] Destructuring in patterns

### 3.4 Exercises
- [ ] Build a `Rectangle` struct with area/perimeter methods
- [ ] Create an enum for different message types
- [ ] Implement a simple state machine using enums

---

## Phase 4: Error Handling (Week 7)

### 4.1 Recoverable Errors
- [ ] `Result<T, E>` in depth
- [ ] `unwrap()` and `expect()` (when to use/avoid)
- [ ] The `?` operator for propagation
- [ ] Custom error types

### 4.2 Unrecoverable Errors
- [ ] `panic!` macro
- [ ] When to panic vs return Result

### 4.3 Exercises
- [ ] Build a file reader with proper error handling
- [ ] Create a config parser that returns meaningful errors

---

## Phase 5: Collections & Iterators (Week 8-9)

### 5.1 Common Collections
- [ ] `Vec<T>` - growable arrays
- [ ] `String` - growable UTF-8 text
- [ ] `HashMap<K, V>` - key-value storage

### 5.2 Iterators
- [ ] Iterator trait and `next()`
- [ ] Iterator adaptors (`map`, `filter`, `fold`)
- [ ] Collecting iterators
- [ ] Creating custom iterators

### 5.3 Exercises
- [ ] Implement a word frequency counter
- [ ] Build a simple in-memory key-value store
- [ ] Process a CSV file using iterators

---

## Phase 6: Generics & Traits (Week 10-11)

### 6.1 Generics
- [ ] Generic functions
- [ ] Generic structs and enums
- [ ] Monomorphization (zero-cost abstractions)

### 6.2 Traits
- [ ] Defining and implementing traits
- [ ] Default implementations
- [ ] Trait bounds (`T: SomeTrait`)
- [ ] `impl Trait` syntax
- [ ] Common traits: `Debug`, `Clone`, `Copy`, `Default`, `PartialEq`, `Eq`

### 6.3 Exercises
- [ ] Create a generic `Pair<T>` struct
- [ ] Implement a `Summarizable` trait for different types
- [ ] Build a generic sorting function with trait bounds

---

## Phase 7: Lifetimes (Week 12)

### 7.1 Lifetime Basics
- [ ] Why lifetimes exist
- [ ] Lifetime annotation syntax (`'a`)
- [ ] Lifetime elision rules

### 7.2 Advanced Lifetimes
- [ ] Lifetimes in structs
- [ ] Multiple lifetime parameters
- [ ] Static lifetime (`'static`)

### 7.3 Exercises
- [ ] Fix lifetime errors in provided broken code
- [ ] Create a struct that holds a reference

---

## Phase 8: Modules & Crates (Week 13)

### 8.1 Module System
- [ ] `mod` keyword and file structure
- [ ] `pub` visibility
- [ ] `use` keyword and paths
- [ ] Re-exporting with `pub use`

### 8.2 Crates & Cargo
- [ ] Library vs binary crates
- [ ] Using external crates from crates.io
- [ ] Cargo workspaces
- [ ] Publishing a crate

---

## Phase 9: Concurrency (Week 14-15)

### 9.1 Threads
- [ ] `std::thread::spawn`
- [ ] `join` handles
- [ ] `move` closures for thread ownership

### 9.2 Message Passing
- [ ] Channels (`mpsc`)
- [ ] Sending and receiving

### 9.3 Shared State
- [ ] `Mutex<T>` for mutual exclusion
- [ ] `Arc<T>` for atomic reference counting
- [ ] `Send` and `Sync` traits

### 9.4 Async/Await (Introduction)
- [ ] `async` functions and `await`
- [ ] Futures
- [ ] Tokio or async-std runtime basics

---

## Phase 10: Projects (Week 16+)

### Beginner Projects
- [ ] Command-line todo app
- [ ] Guessing game with file persistence
- [ ] Simple grep clone (`minigrep`)

### Intermediate Projects
- [ ] HTTP server from scratch
- [ ] JSON parser
- [ ] Simple database (key-value store with persistence)

### Advanced Projects
- [ ] Web API with Axum or Actix
- [ ] CLI tool with `clap`
- [ ] Contribute to an open-source Rust project

---

## Resources

### Official
- [The Rust Book](https://doc.rust-lang.org/book/) - Start here
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises

### Practice
- [Exercism Rust Track](https://exercism.org/tracks/rust)
- [Advent of Code](https://adventofcode.com/) - Solve in Rust
- [LeetCode](https://leetcode.com/) - Algorithm practice in Rust

### Community
- [Rust Users Forum](https://users.rust-lang.org/)
- [r/rust](https://reddit.com/r/rust)
- [This Week in Rust](https://this-week-in-rust.org/)

---

## Progress Tracker

| Phase | Status | Notes |
|-------|--------|-------|
| 1. Foundations | Not Started | |
| 2. Ownership | Not Started | |
| 3. Structuring Data | Not Started | |
| 4. Error Handling | Not Started | |
| 5. Collections | Not Started | |
| 6. Generics & Traits | Not Started | |
| 7. Lifetimes | Not Started | |
| 8. Modules & Crates | Not Started | |
| 9. Concurrency | Not Started | |
| 10. Projects | Not Started | |

---

*Start with Phase 1 and work through sequentially. Each phase builds on the previous.*
