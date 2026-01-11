// =============================================================================
// CHAPTER 3: COMMON PROGRAMMING CONCEPTS
// =============================================================================
// This chapter covers:
// 1. Variables and Mutability
// 2. Data Types (scalar and compound)
// 3. Functions
// 4. Comments
// 5. Control Flow (if, loops)
// =============================================================================

fn main() {
    println!("=== Chapter 3: Common Programming Concepts ===\n");

    variables_and_mutability();
    data_types();
    functions_demo();
    control_flow();
}

// =============================================================================
// PART 1: VARIABLES AND MUTABILITY
// =============================================================================
fn variables_and_mutability() {
    println!("--- Part 1: Variables and Mutability ---\n");

    // ----- Immutable by Default -----
    let x = 5;
    println!("x = {x}");
    // x = 6;  // ERROR! Cannot mutate immutable variable

    // ----- Mutable Variables -----
    let mut y = 5;
    println!("y = {y}");
    y = 6;  // OK! y is mutable
    println!("y after mutation = {y}");

    // ----- Constants -----
    // Must have type annotation, value known at compile time
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // ----- Shadowing -----
    // Reusing a variable name creates a NEW variable
    let z = 5;
    let z = z + 1;  // Shadows previous z
    {
        let z = z * 2;  // Shadows in inner scope
        println!("Inner scope z = {z}");  // 12
    }
    println!("Outer z = {z}");  // 6

    // Shadowing can change types!
    let spaces = "   ";         // &str
    let spaces = spaces.len();  // usize - OK because it's a new variable
    println!("Number of spaces: {spaces}");

    println!();
}

// =============================================================================
// PART 2: DATA TYPES
// =============================================================================
fn data_types() {
    println!("--- Part 2: Data Types ---\n");

    // ----- Scalar Types -----
    // Integers: i8, i16, i32, i64, i128, isize (signed)
    //           u8, u16, u32, u64, u128, usize (unsigned)
    let a: i32 = -42;
    let b: u64 = 100;
    let c: isize = -1;  // Pointer-sized (32 or 64 bits)
    println!("Integers: a={a}, b={b}, c={c}");

    // Integer literals
    let decimal = 98_222;      // Underscores for readability
    let hex = 0xff;            // Hexadecimal
    let octal = 0o77;          // Octal
    let binary = 0b1111_0000;  // Binary
    let byte = b'A';           // Byte (u8 only)
    println!("Literals: {decimal}, {hex}, {octal}, {binary}, {byte}");

    // Floating point: f32, f64 (default)
    let x = 2.0;      // f64 (default)
    let y: f32 = 3.0; // f32
    println!("Floats: x={x}, y={y}");

    // Boolean
    let t = true;
    let f: bool = false;
    println!("Booleans: t={t}, f={f}");

    // Character (4 bytes, Unicode)
    let c = 'z';
    let heart = '❤';
    let emoji = '🦀';
    println!("Characters: {c}, {heart}, {emoji}");

    // ----- Compound Types -----

    // Tuple: fixed-length, different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;  // Destructuring
    println!("Tuple destructured: x={x}, y={y}, z={z}");
    println!("Tuple access: tup.0={}, tup.1={}, tup.2={}", tup.0, tup.1, tup.2);

    // Array: fixed-length, same type, stack-allocated
    let arr = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];  // With type annotation
    let arr3 = [3; 5];  // [3, 3, 3, 3, 3]
    println!("Array: {:?}", arr);
    println!("Array element: arr[0] = {}", arr[0]);
    println!("Repeated array: {:?}", arr3);
    let _ = arr2;  // Suppress unused warning

    println!();
}

// =============================================================================
// PART 3: FUNCTIONS
// =============================================================================
fn functions_demo() {
    println!("--- Part 3: Functions ---\n");

    // Function with parameters
    print_value(5);

    // Function with multiple parameters
    print_labeled_measurement(5, 'h');

    // Function with return value
    let x = five();
    println!("five() returned: {x}");

    // Function with parameter and return
    let y = plus_one(5);
    println!("plus_one(5) = {y}");

    // Expression vs Statement
    let z = {
        let inner = 3;
        inner + 1  // Expression - no semicolon, returns value
    };
    println!("Block expression: {z}");

    println!();
}

fn print_value(x: i32) {
    println!("The value is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5  // Implicit return - no semicolon
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// =============================================================================
// PART 4: CONTROL FLOW
// =============================================================================
fn control_flow() {
    println!("--- Part 4: Control Flow ---\n");

    // ----- if Expressions -----
    let number = 6;

    if number % 4 == 0 {
        println!("{number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("{number} is divisible by 2");
    } else {
        println!("{number} is not divisible by 4, 3, or 2");
    }

    // if in a let statement
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("if expression result: {result}");

    // ----- Loops -----

    // loop - infinite until break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // break with return value
        }
    };
    println!("loop result: {result}");

    // Loop labels - for nested loops
    let mut count = 0;
    'outer: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("  remaining = {remaining}");
            if remaining == 9 {
                break;  // Breaks inner loop
            }
            if count == 2 {
                break 'outer;  // Breaks outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("while: {number}!");
        number -= 1;
    }
    println!("LIFTOFF!");

    // for loop - iterating over a collection
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("for element: {element}");
    }

    // for with range
    for number in 1..4 {  // 1, 2, 3 (exclusive end)
        println!("range: {number}");
    }

    for number in (1..=4).rev() {  // 4, 3, 2, 1 (inclusive, reversed)
        println!("reversed: {number}");
    }

    println!();
}

// =============================================================================
// KEY CONCEPTS SUMMARY
// =============================================================================
//
// VARIABLES:
// | Keyword     | Mutability  | Shadowing | Type Change |
// |-------------|-------------|-----------|-------------|
// | let         | Immutable   | Yes       | Yes         |
// | let mut     | Mutable     | Yes       | No*         |
// | const       | Immutable   | No        | No          |
//
// SCALAR TYPES:
// | Type    | Examples                      | Default |
// |---------|-------------------------------|---------|
// | Integer | i8, i32, u64, isize, usize    | i32     |
// | Float   | f32, f64                      | f64     |
// | Boolean | bool                          | -       |
// | Char    | char (4 bytes, Unicode)       | -       |
//
// COMPOUND TYPES:
// | Type   | Syntax              | Notes                    |
// |--------|---------------------|--------------------------|
// | Tuple  | (i32, f64, u8)      | Fixed length, mixed types|
// | Array  | [i32; 5]            | Fixed length, same type  |
//
// CONTROL FLOW:
// | Construct | Notes                                        |
// |-----------|----------------------------------------------|
// | if        | Condition must be bool, can return value     |
// | loop      | Infinite, use break to exit (can return)     |
// | while     | Loops while condition is true                |
// | for       | Iterates over iterators/ranges               |
// =============================================================================
