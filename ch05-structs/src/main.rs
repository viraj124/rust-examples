// =============================================================================
// CHAPTER 5: USING STRUCTS TO STRUCTURE RELATED DATA
// =============================================================================
// Structs let you create custom data types that group related values.
// Combined with impl blocks, they enable object-oriented-like behavior.
//
// THREE TYPES OF STRUCTS:
// 1. Named-field structs (most common)
// 2. Tuple structs (named tuples)
// 3. Unit structs (no fields)
// =============================================================================

fn main() {
    println!("=== Chapter 5: Using Structs ===\n");

    defining_structs();
    tuple_structs();
    methods_and_impl();
    associated_functions();
}

// =============================================================================
// PART 1: DEFINING AND INSTANTIATING STRUCTS
// =============================================================================

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn defining_structs() {
    println!("--- Part 1: Defining Structs ---\n");

    // Creating an instance
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {}", user1.username);
    println!("Email: {}", user1.email);

    // Mutable instance
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("newemail@example.com");
    println!("Updated email: {}", user2.email);

    // Field init shorthand
    let email = String::from("shorthand@example.com");
    let username = String::from("shorthanduser");
    let user3 = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
    println!("Shorthand user: {}", user3.username);

    // Struct update syntax
    let user4 = User {
        email: String::from("user4@example.com"),
        ..user1  // Copy remaining fields
    };
    println!("User4 username: {}", user4.username);

    // Debug printing
    println!("\nDebug print: {:?}", user4);
    println!("\nPretty debug:\n{:#?}", user4);

    println!();
}

// =============================================================================
// PART 2: TUPLE STRUCTS AND UNIT STRUCTS
// =============================================================================

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn tuple_structs() {
    println!("--- Part 2: Tuple Structs ---\n");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Destructuring
    let Color(r, g, b) = black;
    println!("Destructured: r={r}, g={g}, b={b}");

    // Unit struct
    let _subject = AlwaysEqual;

    println!();
}

// =============================================================================
// PART 3: METHODS WITH IMPL BLOCKS
// =============================================================================

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

fn methods_and_impl() {
    println!("--- Part 3: Methods ---\n");

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("Area of rect1: {} sq pixels", rect1.area());

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Mutable method
    let mut rect4 = Rectangle { width: 5, height: 10 };
    println!("Before double: {:?}", rect4);
    rect4.double_size();
    println!("After double: {:?}", rect4);

    println!();
}

// =============================================================================
// PART 4: ASSOCIATED FUNCTIONS
// =============================================================================

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn associated_functions() {
    println!("--- Part 4: Associated Functions ---\n");

    let sq = Rectangle::square(25);
    println!("Square: {:?}", sq);

    let rect = Rectangle::new(10, 20);
    println!("New rect: {:?}", rect);

    println!();
}

// =============================================================================
// KEY CONCEPTS SUMMARY
// =============================================================================
//
// STRUCT TYPES:
// | Type        | Syntax                    | Access    |
// |-------------|---------------------------|-----------|
// | Named       | struct Name { f: T }      | name.f    |
// | Tuple       | struct Name(T, U)         | name.0    |
// | Unit        | struct Name;              | N/A       |
//
// IMPL BLOCKS:
// | Function Type      | First Parameter | Called As        |
// |--------------------|-----------------|------------------|
// | Method             | &self           | instance.method()|
// | Method (mutable)   | &mut self       | instance.method()|
// | Associated fn      | (none)          | Type::function() |
//
// COMMON DERIVE TRAITS:
// - #[derive(Debug)]    - Enable {:?} printing
// - #[derive(Clone)]    - Enable .clone()
// - #[derive(PartialEq)]- Enable == comparison
// =============================================================================
