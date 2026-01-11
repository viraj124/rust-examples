// =============================================================================
// CHAPTER 6: ENUMS AND PATTERN MATCHING
// =============================================================================
// Enums define a type by enumerating its possible variants.
// Combined with match, they provide powerful pattern matching.
//
// KEY CONCEPTS:
// 1. Enum variants can hold different types of data
// 2. Option<T> handles nullable values safely
// 3. match must be exhaustive (handle all cases)
// 4. if let provides concise single-pattern matching
// =============================================================================

fn main() {
    println!("=== Chapter 6: Enums and Pattern Matching ===\n");

    defining_enums();
    enums_with_data();
    option_enum();
    match_expressions();
    if_let_syntax();
}

// =============================================================================
// PART 1: DEFINING ENUMS
// =============================================================================

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn defining_enums() {
    println!("--- Part 1: Defining Enums ---\n");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IPv4: {:?}", four);
    println!("IPv6: {:?}", six);

    println!();
}

// =============================================================================
// PART 2: ENUMS WITH DATA
// =============================================================================

#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message called: {:?}", self);
    }
}

fn enums_with_data() {
    println!("--- Part 2: Enums with Data ---\n");

    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write = Message::Write(String::from("hello"));
    let color = Message::ChangeColor(255, 128, 0);

    quit.call();
    move_msg.call();
    write.call();
    color.call();

    println!();
}

// =============================================================================
// PART 3: THE OPTION ENUM
// =============================================================================

fn option_enum() {
    println!("--- Part 3: Option Enum ---\n");

    let some_number = Some(5);
    let some_string = Some("hello");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    // Common Option methods
    let value = Some(10);
    println!("unwrap: {}", value.unwrap());
    println!("unwrap_or: {}", None::<i32>.unwrap_or(42));
    println!("is_some: {}", value.is_some());
    println!("is_none: {}", value.is_none());

    let mapped = value.map(|x| x * 2);
    println!("mapped: {:?}", mapped);

    println!();
}

// =============================================================================
// PART 4: MATCH EXPRESSIONS
// =============================================================================

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

fn match_expressions() {
    println!("--- Part 4: Match Expressions ---\n");

    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("Penny value: {value} cents");

    let quarter = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(quarter);
    println!("Quarter value: {value} cents");

    // Matching Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("plus_one(5): {:?}", six);
    println!("plus_one(None): {:?}", none);

    // Catch-all patterns
    let dice_roll = 9;
    match dice_roll {
        3 => println!("Add fancy hat!"),
        7 => println!("Remove fancy hat!"),
        other => println!("Move {other} spaces"),
    }

    println!();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// =============================================================================
// PART 5: IF LET SYNTAX
// =============================================================================

fn if_let_syntax() {
    println!("--- Part 5: if let Syntax ---\n");

    let config_max = Some(3u8);

    // Concise if let
    if let Some(max) = config_max {
        println!("if let: max is {max}");
    }

    // if let with else
    let coin = Coin::Quarter(UsState::California);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not a quarter");
    }

    // while let
    let mut stack = vec![1, 2, 3];
    print!("while let: ");
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();

    println!();
}

// =============================================================================
// KEY CONCEPTS SUMMARY
// =============================================================================
//
// ENUM VARIANTS:
// | Type          | Syntax                    | Example              |
// |---------------|---------------------------|----------------------|
// | Unit          | Variant                   | None, Quit           |
// | Tuple         | Variant(T, U)             | Some(5), Move(1,2)   |
// | Struct        | Variant { a: T, b: U }    | Move { x: 1, y: 2 }  |
//
// OPTION<T>:
// | Method           | Returns     | On None          |
// |------------------|-------------|------------------|
// | unwrap()         | T           | panics           |
// | unwrap_or(def)   | T           | returns def      |
// | is_some()        | bool        | false            |
// | is_none()        | bool        | true             |
// | map(f)           | Option<U>   | None             |
//
// MATCH PATTERNS:
// | Pattern          | Description                          |
// |------------------|--------------------------------------|
// | Variant          | Match specific variant               |
// | Variant(x)       | Extract inner value                  |
// | other            | Catch-all, binds value               |
// | _                | Catch-all, ignores value             |
// =============================================================================
