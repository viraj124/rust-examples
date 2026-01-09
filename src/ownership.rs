// =============================================================================
// RUST OWNERSHIP - The Core Memory Safety System
// =============================================================================
// Ownership is Rust's most unique feature and enables memory safety without GC.
//
// THE THREE RULES OF OWNERSHIP:
// 1. Each value in Rust has exactly ONE owner
// 2. When the owner goes out of scope, the value is dropped (freed)
// 3. Ownership can be transferred (moved) or borrowed (referenced)
//
// BORROWING RULES:
// - You can have EITHER:
//   * One mutable reference (&mut T), OR
//   * Any number of immutable references (&T)
// - References must always be valid (no dangling references)
// =============================================================================

pub(crate) fn ownership() {
    // =========================================================================
    // STACK vs HEAP
    // =========================================================================
    // Stack: Fixed-size data, fast, LIFO (Last In First Out)
    //   - Integers, floats, booleans, chars, fixed arrays, tuples
    //   - String literals (&str) - stored in binary, referenced on stack
    //
    // Heap: Dynamic-size data, slower, requires allocation
    //   - String, Vec, Box, HashMap, etc.

    // String literals are stored in the binary and are immutable
    // let s = "34";         // &str - reference to static string
    // let s1: &str = s;     // Another reference - no allocation
    // s = "45";             // ERROR! Can't mutate - it's a reference to static data

    // String is heap-allocated and can be mutated
    // let mut s1 = String::from("s");
    // s1.push_str("sss");   // OK! String owns its data and can grow

    // =========================================================================
    // MOVE SEMANTICS
    // =========================================================================
    // When you assign a heap value to another variable, ownership MOVES.
    // The original variable becomes invalid!

    // let s12 = String::from("hello");
    // let s22 = s12;                    // s12 is MOVED to s22
    // println!("{s12}, world!");        // ERROR! s12 is no longer valid

    // WHY? To prevent double-free errors!
    // If both s12 and s22 owned the same memory, both would try to free it.

    // =========================================================================
    // CLONE - Deep Copy
    // =========================================================================
    // Use clone() to create an independent copy (both variables valid)

    // let s12 = String::from("hello");
    // let s22 = s12.clone();           // Deep copy - new heap allocation
    // println!("{s12}, world!");       // OK! s12 is still valid

    // NOTE: clone() can be expensive for large data!

    // =========================================================================
    // COPY TRAIT - Stack-Only Types
    // =========================================================================
    // Types that implement Copy are automatically copied (not moved).
    // These are small, fixed-size types stored entirely on the stack:
    //   - Integers (i32, u64, etc.)
    //   - Floating point (f32, f64)
    //   - Booleans (bool)
    //   - Characters (char)
    //   - Tuples of Copy types

    // let x = 5;
    // let y = x;           // x is COPIED, not moved!
    // println!("{x}");     // OK! x is still valid

    // =========================================================================
    // REFERENCES AND BORROWING
    // =========================================================================
    // References let you use a value without taking ownership.
    // &T  = immutable reference (can read)
    // &mut T = mutable reference (can read and write)

    // IMMUTABLE REFERENCES - Can have many
    // let y = 7;
    // let z = &y;          // Borrow y immutably
    // let w = &y;          // Another immutable borrow - OK!
    // println!("{z} {w}"); // Both valid

    // MUTABLE REFERENCES - Can have only ONE at a time
    // let mut x = 5;
    // let r1 = &mut x;
    // let r2 = &mut x;     // ERROR! Can't have two mutable references
    // println!("{}", r1);  // Would fail to compile

    // WHY? Prevents data races at compile time!

    // =========================================================================
    // MIXING MUTABLE AND IMMUTABLE REFERENCES
    // =========================================================================
    // You cannot have mutable AND immutable references simultaneously
    // (while they're both being used)

    // let mut s = String::from("hello");
    // let r1 = &s;         // Immutable borrow
    // let r2 = &s;         // Another immutable borrow - OK
    // println!("{r1} and {r2}");
    // // r1 and r2 go out of scope here (last use)

    // let r3 = &mut s;     // OK! No immutable borrows active now
    // println!("{r3}");

    // =========================================================================
    // OWNERSHIP AND FUNCTIONS
    // =========================================================================
    // Passing a value to a function MOVES it (unless it's Copy)

    // let s = String::from("hello");
    // takes_ownership(s);   // s is moved into the function
    // println!("{s}");      // ERROR! s is no longer valid

    // let x = 5;
    // makes_copy(x);        // x is copied (i32 implements Copy)
    // println!("{x}");      // OK! x is still valid

    // =========================================================================
    // REFERENCES IN FUNCTIONS
    // =========================================================================
    // Use references to avoid moving ownership

    // let mut s = String::from("hello");
    // calculate_length(&s); // Borrow immutably - s still valid
    // println!("{s}");      // OK!

    // change(&mut s);       // Borrow mutably - function can modify
    // println!("{s}");      // Prints: "hellohey"

    // =========================================================================
    // DANGLING REFERENCES
    // =========================================================================
    // Rust prevents returning references to local variables

    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s   // ERROR! s is dropped when function returns
    // }        // Reference would point to freed memory!

    // Solution: Return the owned value instead
    // fn no_dangle() -> String {
    //     let s = String::from("hello");
    //     s    // Ownership is transferred to caller
    // }
}

// =============================================================================
// HELPER FUNCTIONS DEMONSTRATING OWNERSHIP
// =============================================================================

/// Takes ownership of a String - the String is moved into this function
fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string goes out of scope and `drop` is called - memory freed

/// Makes a copy of an i32 - the original remains valid in the caller
fn makes_copy(some_integer: i32) {
    // Can't mutate parameter - it's not marked `mut`
    // some_integer = 10;  // ERROR!
    let _x = some_integer;
    println!("{}", _x);
} // some_integer goes out of scope. Nothing special - just stack memory

/// Borrows a String immutably - caller keeps ownership
fn calculate_length(val: &String) {
    let _c = val.len();
    // Can read val, but can't modify it
    // val.push_str("!");  // ERROR! Can't mutate through immutable reference
}

/// Borrows a String mutably - can modify the caller's data
fn change(val: &mut String) {
    val.push_str("hey");
    // Caller's String is now modified!
}

// =============================================================================
// DANGLING REFERENCE EXAMPLE (Commented out - would not compile)
// =============================================================================
// This demonstrates what Rust prevents:
//
// fn dangle() -> &'static String {
//     let s = String::from("hello");  // s is created here
//
//     &s // ERROR: returning reference to local variable
//        // s will be dropped when this function ends
//        // The reference would point to freed memory!
// }
//
// SOLUTION: Return the owned value instead:
// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s  // Ownership transferred to caller - no dangling reference!
// }

// =============================================================================
// OWNERSHIP SUMMARY
// =============================================================================
//
// | Operation           | What Happens                    | Original Valid? |
// |--------------------|---------------------------------|-----------------|
// | let y = x          | Move (heap) or Copy (stack)     | Copy: Yes       |
// |                    |                                 | Move: No        |
// | let y = x.clone()  | Deep copy (heap data)           | Yes             |
// | let y = &x         | Immutable borrow                | Yes             |
// | let y = &mut x     | Mutable borrow                  | Yes (exclusive) |
// | fn(x)              | Move or Copy into function      | Depends on type |
// | fn(&x)             | Borrow for function             | Yes             |
//
// MEMORY SAFETY GUARANTEES:
// - No dangling pointers (references always valid)
// - No double frees (one owner = one free)
// - No data races (mutable XOR shared, never both)
// =============================================================================
