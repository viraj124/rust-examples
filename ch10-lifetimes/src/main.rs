// =============================================================================
// RUST LIFETIMES - Ensuring References Are Valid
// =============================================================================
// Lifetimes are Rust's way of ensuring that references are always valid.
// They tell the compiler how long references should live.
//
// KEY CONCEPTS:
// 1. Every reference has a lifetime (usually inferred)
// 2. Lifetime annotations don't change how long data lives
// 3. They help the compiler verify that references are valid
// 4. Syntax: 'a (apostrophe + name, 'a is convention)
// =============================================================================

use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    // Both references must live at least as long as `result` is used
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // =========================================================================
    // WHY LIFETIMES ARE NEEDED
    // =========================================================================
    // The compiler can't tell if x or y's reference is returned.
    // Without lifetimes, it can't guarantee the returned reference is valid.
    //
    // fn longest(x: &str, y: &str) -> &str {  // ERROR! Missing lifetime
    //     if x.len() > y.len() { x } else { y }
    // }

    // =========================================================================
    // LIFETIMES IN STRUCTS
    // =========================================================================
    // When a struct holds references, it needs lifetime annotations.
    // This ensures the struct can't outlive the data it references.

    let val = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = val.split('.').next().unwrap();
    let person = Person { name: first_sentence };
    println!("Person name: {}", person.get_name("test"));

    // =========================================================================
    // COMBINING GENERICS, TRAITS, AND LIFETIMES
    // =========================================================================
    let result2 = longest_with_announcement(
        string1.as_str(),
        string2,
        "Comparing strings!",
    );
    println!("The longest string is {result2}");
}

// =============================================================================
// STRUCT WITH LIFETIME
// =============================================================================
// Person holds a reference (&str), so we need a lifetime parameter.
// 'a means: Person cannot outlive the string it references.

struct Person<'a> {
    name: &'a str,  // This reference must live at least as long as Person
}

// =============================================================================
// LIFETIME ELISION RULES
// =============================================================================
// Rust has 3 rules that let you omit lifetimes in common cases:
//
// 1. Each reference parameter gets its own lifetime
//    fn foo(x: &str, y: &str) -> fn foo<'a, 'b>(x: &'a str, y: &'b str)
//
// 2. If there's exactly one input lifetime, it's assigned to all outputs
//    fn foo(x: &str) -> &str  ->  fn foo<'a>(x: &'a str) -> &'a str
//
// 3. If &self or &mut self, self's lifetime is assigned to all outputs
//    fn method(&self, x: &str) -> &str  (output gets self's lifetime)

impl<'a> Person<'a> {
    // Rule 3 applies: return lifetime = self's lifetime
    // No explicit lifetime needed on return type!
    fn get_name(&self, _val: &str) -> &str {
        self.name
    }
}

// =============================================================================
// FUNCTION WITH LIFETIME ANNOTATIONS
// =============================================================================
// 'a means: the returned reference lives as long as the SHORTER of x and y.
// Both x and y must be valid for at least 'a, and so must the return value.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// =============================================================================
// COMBINING GENERICS, TRAITS, AND LIFETIMES
// =============================================================================
// All three can be used together:
// - 'a: lifetime parameter
// - T: generic type parameter
// - where T: Display: trait bound

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {ann}");
    if x.len() > y.len() { x } else { y }
}

// =============================================================================
// COMMON LIFETIME PATTERNS
// =============================================================================
//
// PATTERN 1: Return one of the inputs
// fn first<'a>(x: &'a str, y: &str) -> &'a str { x }
// Only x's lifetime matters for return value
//
// PATTERN 2: Struct holding reference
// struct Excerpt<'a> { part: &'a str }
// Struct can't outlive the string it references
//
// PATTERN 3: Multiple references with same lifetime
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
// Return value is valid while BOTH inputs are valid
//
// PATTERN 4: Different lifetimes
// fn complex<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
// Only x's lifetime affects return value
// =============================================================================

// =============================================================================
// STATIC LIFETIME
// =============================================================================
// 'static means the reference lives for the entire program duration.
// String literals have 'static lifetime (stored in binary).
//
// let s: &'static str = "I live forever!";
//
// Be careful: &'static T doesn't mean the data is immortal,
// it means the reference CAN live that long if needed.
// =============================================================================

// =============================================================================
// WHAT LIFETIMES DON'T DO
// =============================================================================
// - They DON'T change how long data lives
// - They DON'T allocate or free memory
// - They're just annotations that help the compiler verify safety
//
// This won't compile - can't return reference to local data:
// fn dangling<'a>() -> &'a str {
//     let s = String::from("hello");
//     s.as_str()  // ERROR! s is dropped, reference would be invalid
// }
// =============================================================================
