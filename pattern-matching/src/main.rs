// =============================================================================
// CHAPTER 19: PATTERNS AND MATCHING
// =============================================================================
// Patterns are a special syntax for matching against the structure of types.
// Used in: match, if let, while let, for loops, let statements, function params
//
// PATTERN TYPES:
// - Irrefutable: Always match (e.g., let x = 5)
// - Refutable: Might fail to match (e.g., if let Some(x) = value)
// =============================================================================

fn main() {
    println!("=== Chapter 19: Patterns and Matching ===\n");

    // =========================================================================
    // PART 1: WHERE PATTERNS ARE USED
    // =========================================================================
    patterns_locations();

    // =========================================================================
    // PART 2: PATTERN SYNTAX
    // =========================================================================
    pattern_syntax();

    // =========================================================================
    // PART 3: DESTRUCTURING
    // =========================================================================
    destructuring_examples();

    // =========================================================================
    // PART 4: PATTERN GUARDS
    // =========================================================================
    pattern_guards();

    // =========================================================================
    // PART 5: @ BINDINGS
    // =========================================================================
    at_bindings();
}

// =============================================================================
// PART 1: ALL PLACES PATTERNS CAN BE USED
// =============================================================================

fn patterns_locations() {
    println!("--- Part 1: Where Patterns Are Used ---\n");

    // ----- 1. match arms -----
    let x = 1;
    match x {
        1 => println!("match: one"),
        2 => println!("match: two"),
        _ => println!("match: anything else"),
    }

    // ----- 2. if let (refutable pattern) -----
    let favorite_color: Option<&str> = Some("blue");
    if let Some(color) = favorite_color {
        println!("if let: Your favorite color is {color}");
    }

    // ----- 3. while let -----
    let mut stack = vec![1, 2, 3];
    print!("while let: ");
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();

    // ----- 4. for loops -----
    let v = vec!['a', 'b', 'c'];
    print!("for loop destructuring: ");
    for (index, value) in v.iter().enumerate() {
        print!("({index},{value}) ");
    }
    println!();

    // ----- 5. let statements -----
    let (a, b, c) = (1, 2, 3);
    println!("let destructuring: a={a}, b={b}, c={c}");

    // ----- 6. function parameters -----
    print_coordinates(&(10, 20));

    println!();
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("function param pattern: x={x}, y={y}");
}

// =============================================================================
// PART 2: PATTERN SYNTAX
// =============================================================================

fn pattern_syntax() {
    println!("--- Part 2: Pattern Syntax ---\n");

    // ----- Matching Literals -----
    let x = 1;
    match x {
        1 => println!("Literal: one"),
        2 => println!("Literal: two"),
        _ => println!("Literal: other"),
    }

    // ----- Matching Named Variables -----
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {y}"),  // y shadows outer y!
        _ => println!("Default"),
    }
    println!("Outer y still = {y}");  // Original y unchanged

    // ----- Multiple Patterns with | -----
    let x = 1;
    match x {
        1 | 2 => println!("Or pattern: one or two"),
        3 => println!("Or pattern: three"),
        _ => println!("Or pattern: other"),
    }

    // ----- Ranges with ..= -----
    let x = 5;
    match x {
        1..=5 => println!("Range: one through five"),
        _ => println!("Range: other"),
    }

    let c = 'c';
    match c {
        'a'..='j' => println!("Char range: early ASCII letter"),
        'k'..='z' => println!("Char range: late ASCII letter"),
        _ => println!("Char range: something else"),
    }

    // ----- Ignoring Values with _ -----
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Ignore with _: {first}, {third}, {fifth}");
        }
    }

    // ----- Ignoring with .. -----
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => {
            println!("Ignore with ..: first={first}, last={last}");
        }
    }

    println!();
}

// =============================================================================
// PART 3: DESTRUCTURING
// =============================================================================

struct Point {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructuring_examples() {
    println!("--- Part 3: Destructuring ---\n");

    // ----- Destructuring Structs -----
    let p = Point { x: 10, y: 20 };

    // Full destructure
    let Point { x: a, y: b } = p;
    println!("Struct full: a={a}, b={b}");

    // Shorthand when variable names match field names
    let Point { x, y } = p;
    println!("Struct shorthand: x={x}, y={y}");

    // Destructure in match
    match p {
        Point { x: 0, y } => println!("Match struct: On y-axis at {y}"),
        Point { x, y: 0 } => println!("Match struct: On x-axis at {x}"),
        Point { x, y } => println!("Match struct: At ({x}, {y})"),
    }

    // ----- Destructuring Enums -----
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        Message::Quit => println!("Enum: Quit"),
        Message::Move { x, y } => println!("Enum: Move to ({x}, {y})"),
        Message::Write(text) => println!("Enum: Write '{text}'"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Enum: Change to RGB({r}, {g}, {b})");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Enum: Change to HSV({h}, {s}, {v})");
        }
    }

    // ----- Nested Destructuring -----
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 5, y: -5 });
    println!("Nested: feet={feet}, inches={inches}, x={x}, y={y}");

    // ----- Destructuring References -----
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    // &Point in pattern destructures the reference
    let sum_of_y: i32 = points.iter().map(|&Point { x: _, y }| y).sum();
    println!("Sum of y values: {sum_of_y}");

    println!();
}

// =============================================================================
// PART 4: PATTERN GUARDS - Extra Conditions
// =============================================================================

fn pattern_guards() {
    println!("--- Part 4: Pattern Guards ---\n");

    // Pattern guard = if condition after pattern
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("Guard: {x} is even"),
        Some(x) => println!("Guard: {x} is odd"),
        None => println!("Guard: No value"),
    }

    // Using guards with multiple patterns
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("Multi + guard: yes"),
        _ => println!("Multi + guard: no"),
    }

    // Using outer variables in guards (without shadowing)
    let x = Some(5);
    let y = 10;

    match x {
        Some(n) if n == y => println!("Variable guard: Matched, n = {n}"),
        _ => println!("Variable guard: No match, y = {y}"),
    }

    println!();
}

// =============================================================================
// PART 5: @ BINDINGS - Bind a Value While Testing
// =============================================================================

fn at_bindings() {
    println!("--- Part 5: @ Bindings ---\n");

    enum Msg {
        Hello { id: i32 },
    }

    let msg = Msg::Hello { id: 5 };

    // @ lets us test a value AND capture it in a variable
    match msg {
        Msg::Hello { id: id_variable @ 3..=7 } => {
            // id_variable holds the value, and we know it's in range 3-7
            println!("@ binding: Found id in range: {id_variable}");
        }
        Msg::Hello { id: 10..=12 } => {
            // We can test the range but can't use the value
            println!("@ binding: Found id in range 10-12");
        }
        Msg::Hello { id } => {
            println!("@ binding: Found other id: {id}");
        }
    }

    // @ with structs
    let p = Point3D { x: 0, y: 5, z: 10 };

    match p {
        Point3D { x: 0, y: y_val @ 1..=10, z } => {
            println!("@ struct: y={y_val} (in range 1-10), z={z}");
        }
        _ => println!("@ struct: No match"),
    }

    println!();
}

// =============================================================================
// KEY CONCEPTS SUMMARY
// =============================================================================
//
// PATTERN LOCATIONS:
// | Location          | Refutable? | Example                        |
// |-------------------|------------|--------------------------------|
// | match arm         | Yes        | match x { Some(y) => ... }     |
// | if let            | Yes        | if let Some(y) = x { }         |
// | while let         | Yes        | while let Some(y) = iter.next()|
// | for loop          | No         | for (i, v) in enumerate()      |
// | let statement     | No         | let (x, y) = (1, 2)            |
// | function param    | No         | fn foo((x, y): (i32, i32))     |
//
// PATTERN SYNTAX:
// | Pattern           | Example                | Meaning                  |
// |-------------------|------------------------|--------------------------|
// | Literal           | 1, "hello"             | Match exact value        |
// | Variable          | x                      | Bind to variable         |
// | Wildcard          | _                      | Match anything, ignore   |
// | Rest              | ..                     | Ignore remaining fields  |
// | Or                | 1 | 2                  | Match either pattern     |
// | Range             | 1..=5, 'a'..='z'       | Match range of values    |
// | Destructure       | Point { x, y }         | Extract fields           |
// | Guard             | x if x > 0             | Additional condition     |
// | @ binding         | id @ 1..=5             | Test and bind            |
//
// REFUTABLE VS IRREFUTABLE:
// - Irrefutable: Guaranteed to match (let, for, function params)
// - Refutable: Might not match (match, if let, while let)
// =============================================================================
