// =============================================================================
// CHAPTER 4: UNDERSTANDING OWNERSHIP
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

fn main() {
    println!("=== Chapter 4: Understanding Ownership ===\n");

    ownership_basics();
    move_semantics();
    clone_and_copy();
    references_and_borrowing();
    mutable_references();
    slices();
}

// =============================================================================
// PART 1: OWNERSHIP BASICS
// =============================================================================
fn ownership_basics() {
    println!("--- Part 1: Ownership Basics ---\n");

    // Stack vs Heap
    // Stack: Fixed-size data, fast, LIFO
    // Heap: Dynamic-size data, slower, requires allocation

    // String literals are stored in binary - immutable
    let s1 = "hello";  // &str - reference to static string
    println!("String literal (stack): {s1}");

    // String is heap-allocated and can grow
    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("String (heap): {s2}");

    // Scope determines when values are dropped
    {
        let s3 = String::from("temporary");
        println!("Inside scope: {s3}");
    }  // s3 is dropped here - memory freed

    println!();
}

// =============================================================================
// PART 2: MOVE SEMANTICS
// =============================================================================
fn move_semantics() {
    println!("--- Part 2: Move Semantics ---\n");

    // For heap data, assignment MOVES ownership
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2

    // println!("{s1}");  // ERROR! s1 is no longer valid
    println!("After move, s2 = {s2}");

    // Functions also take ownership
    let s3 = String::from("function");
    takes_ownership(s3);
    // println!("{s3}");  // ERROR! s3 was moved into the function

    // Return values transfer ownership back
    let s4 = gives_ownership();
    println!("Received ownership: {s4}");

    let s5 = String::from("round trip");
    let s6 = takes_and_gives_back(s5);
    println!("Round trip: {s6}");

    println!();
}

fn takes_ownership(s: String) {
    println!("takes_ownership received: {s}");
}

fn gives_ownership() -> String {
    String::from("yours now")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

// =============================================================================
// PART 3: CLONE AND COPY
// =============================================================================
fn clone_and_copy() {
    println!("--- Part 3: Clone and Copy ---\n");

    // Clone: Deep copy of heap data
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Deep copy - both valid
    println!("Cloned: s1={s1}, s2={s2}");

    // Copy trait: Stack-only types are copied automatically
    let x = 5;
    let y = x;  // x is COPIED, not moved
    println!("Copied: x={x}, y={y}");

    // Tuples of Copy types are also Copy
    let point = (3, 4);
    let point2 = point;
    println!("Tuple copy: point={:?}, point2={:?}", point, point2);

    println!();
}

// =============================================================================
// PART 4: REFERENCES AND BORROWING
// =============================================================================
fn references_and_borrowing() {
    println!("--- Part 4: References and Borrowing ---\n");

    let s1 = String::from("hello");

    // &s1 creates a reference - borrowing, not owning
    let len = calculate_length(&s1);
    println!("Length of '{s1}' is {len}");

    // Multiple immutable references are allowed
    let r1 = &s1;
    let r2 = &s1;
    println!("Multiple refs: r1={r1}, r2={r2}");

    println!();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// =============================================================================
// PART 5: MUTABLE REFERENCES
// =============================================================================
fn mutable_references() {
    println!("--- Part 5: Mutable References ---\n");

    // &mut creates a mutable reference
    let mut s = String::from("hello");
    change(&mut s);
    println!("After change: {s}");

    // RULE: Only ONE mutable reference at a time
    let mut s2 = String::from("hello");
    let r1 = &mut s2;
    // let r2 = &mut s2;  // ERROR!
    println!("Mutable ref: {r1}");

    // RULE: Can't mix mutable and immutable references
    let mut s3 = String::from("hello");
    let r1 = &s3;
    let r2 = &s3;
    println!("Immutable refs: {r1}, {r2}");
    // r1 and r2 scope ends here

    let r3 = &mut s3;  // OK now
    println!("Mutable ref: {r3}");

    println!();
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// =============================================================================
// PART 6: SLICES
// =============================================================================
fn slices() {
    println!("--- Part 6: Slices ---\n");

    let s = String::from("hello world");

    // String slices: &str
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Slices: '{hello}' and '{world}'");

    // Shorthand
    let hello = &s[..5];    // From beginning
    let world = &s[6..];    // To end
    let full = &s[..];      // Entire string
    println!("Shorthand: '{hello}', '{world}', '{full}'");

    // String literals ARE slices
    let literal = "hello world";  // Type is &str
    println!("Literal is &str: {literal}");

    // First word function
    let first = first_word(&s);
    println!("First word: {first}");

    // Array slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("Array slice: {:?}", slice);

    println!();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// =============================================================================
// KEY CONCEPTS SUMMARY
// =============================================================================
//
// OWNERSHIP RULES:
// 1. Each value has one owner
// 2. Value dropped when owner goes out of scope
// 3. Ownership can move or be borrowed
//
// MOVE VS COPY:
// | Type            | Behavior | Notes                        |
// |-----------------|----------|------------------------------|
// | Heap (String)   | Move     | Original becomes invalid     |
// | Stack (i32)     | Copy     | Both remain valid            |
// | .clone()        | Copy     | Explicit deep copy           |
//
// REFERENCES:
// | Syntax     | Type       | Can Read | Can Write | Count    |
// |------------|------------|----------|-----------|----------|
// | &T         | Immutable  | Yes      | No        | Many     |
// | &mut T     | Mutable    | Yes      | Yes       | One      |
//
// SLICES:
// | Type    | Syntax        | Notes                    |
// |---------|---------------|--------------------------|
// | &str    | &s[0..5]      | String slice             |
// | &[T]    | &arr[1..3]    | Array slice              |
// =============================================================================
