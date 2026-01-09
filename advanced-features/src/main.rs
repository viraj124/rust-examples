// =============================================================================
// CHAPTER 20: ADVANCED FEATURES
// =============================================================================
// This chapter covers advanced Rust features:
// 1. Unsafe Rust - bypass safety checks for low-level operations
// 2. Advanced Traits - associated types, default params, supertraits
// 3. Advanced Types - newtypes, type aliases, never type
// 4. Advanced Functions - function pointers, returning closures
// 5. Macros - metaprogramming with declarative and procedural macros
// =============================================================================

use std::ops::Add;
use std::fmt;

fn main() {
    println!("=== Chapter 20: Advanced Features ===\n");

    // =========================================================================
    // PART 1: UNSAFE RUST
    // =========================================================================
    unsafe_examples();

    // =========================================================================
    // PART 2: ADVANCED TRAITS
    // =========================================================================
    advanced_traits();

    // =========================================================================
    // PART 3: ADVANCED TYPES
    // =========================================================================
    advanced_types();

    // =========================================================================
    // PART 4: ADVANCED FUNCTIONS AND CLOSURES
    // =========================================================================
    advanced_functions();

    // =========================================================================
    // PART 5: MACROS
    // =========================================================================
    macro_examples();
}

// =============================================================================
// PART 1: UNSAFE RUST
// =============================================================================
// Unsafe allows 5 things that safe Rust doesn't:
// 1. Dereference a raw pointer
// 2. Call an unsafe function or method
// 3. Access or modify a mutable static variable
// 4. Implement an unsafe trait
// 5. Access fields of a union

fn unsafe_examples() {
    println!("--- Part 1: Unsafe Rust ---\n");

    // ----- Raw Pointers -----
    // Two kinds: *const T (immutable) and *mut T (mutable)
    // Unlike references: can be null, don't guarantee valid memory, no borrow checking

    let mut num = 5;

    // Creating raw pointers is SAFE
    let r1 = &num as *const i32;      // immutable raw pointer
    let r2 = &mut num as *mut i32;    // mutable raw pointer

    // DEREFERENCING raw pointers requires unsafe
    unsafe {
        println!("Raw pointer r1: {}", *r1);
        println!("Raw pointer r2: {}", *r2);
    }

    // ----- Calling Unsafe Functions -----
    unsafe fn dangerous() {
        println!("This is an unsafe function");
    }

    unsafe {
        dangerous();
    }

    // ----- Safe Abstraction over Unsafe Code -----
    // split_at_mut is safe API but uses unsafe internally
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = v.split_at_mut(3);
    println!("Split: {:?} | {:?}", left, right);

    // Our own split_at_mut implementation
    fn split_at_mut_custom(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();  // Get raw pointer

        assert!(mid <= len);

        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut arr = [1, 2, 3, 4, 5];
    let (a, b) = split_at_mut_custom(&mut arr, 2);
    println!("Custom split: {:?} | {:?}", a, b);

    // ----- Static Variables -----
    // Static variables have 'static lifetime and fixed memory address
    // Mutable statics are unsafe because of potential data races

    static HELLO_WORLD: &str = "Hello, world!";
    static mut COUNTER: u32 = 0;

    println!("Static string: {}", HELLO_WORLD);

    // Accessing mutable static requires unsafe
    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }

    // ----- Extern Functions (FFI) -----
    // Call C library functions
    // In Rust 2024 edition, extern blocks must be marked unsafe
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("C abs(-3) = {}", abs(-3));
    }

    println!();
}

// =============================================================================
// PART 2: ADVANCED TRAITS
// =============================================================================

fn advanced_traits() {
    println!("--- Part 2: Advanced Traits ---\n");

    // ----- Associated Types -----
    // Define a placeholder type in the trait definition
    // Implementor specifies the concrete type

    trait Container {
        type Item;  // Associated type
        fn get(&self, index: usize) -> Option<&Self::Item>;
    }

    struct NumberBox {
        items: Vec<i32>,
    }

    impl Container for NumberBox {
        type Item = i32;  // Specify the type once

        fn get(&self, index: usize) -> Option<&Self::Item> {
            self.items.get(index)
        }
    }

    let nb = NumberBox { items: vec![1, 2, 3] };
    println!("Associated type: {:?}", nb.get(1));

    // ----- Default Generic Type Parameters -----
    // Add trait has default: type Rhs = Self

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    println!("Point addition: {:?}", p1 + p2);

    // Adding different types
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    let mm = Millimeters(500);
    let m = Meters(1);
    let result = mm + m;
    println!("500mm + 1m = {}mm", result.0);

    // ----- Fully Qualified Syntax for Disambiguation -----
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("*Levitates*");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    person.fly();              // Calls Human::fly
    Pilot::fly(&person);       // Calls Pilot::fly
    Wizard::fly(&person);      // Calls Wizard::fly

    // ----- Associated Functions (no self) need fully qualified syntax -----
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("Dog::baby_name() = {}", Dog::baby_name());
    // To call trait method: <Type as Trait>::method()
    println!("<Dog as Animal>::baby_name() = {}", <Dog as Animal>::baby_name());

    // ----- Supertraits -----
    // OutlinePrint requires Display
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();  // Uses Display
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("* {} *", output);
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Position { x: i32, y: i32 }

    impl fmt::Display for Position {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Position {}

    let pos = Position { x: 10, y: 20 };
    pos.outline_print();

    println!();
}

// =============================================================================
// PART 3: ADVANCED TYPES
// =============================================================================

fn advanced_types() {
    println!("--- Part 3: Advanced Types ---\n");

    // ----- Newtype Pattern -----
    // Wrap a type to provide different behavior or satisfy orphan rule

    struct Kilometers(f64);
    struct Miles(f64);

    impl Kilometers {
        fn to_miles(&self) -> Miles {
            Miles(self.0 * 0.621371)
        }
    }

    let km = Kilometers(100.0);
    let miles = km.to_miles();
    println!("Newtype: 100 km = {:.2} miles", miles.0);

    // Newtype to implement external trait on external type
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("Wrapper Display: {}", w);

    // ----- Type Aliases -----
    // Create a new name for an existing type

    type Thunk = Box<dyn Fn() + Send + 'static>;

    fn takes_long_type(_f: Thunk) {
        // Much cleaner than Box<dyn Fn() + Send + 'static>
    }

    type Result<T> = std::result::Result<T, std::io::Error>;
    // Now Result<i32> means std::result::Result<i32, std::io::Error>

    println!("Type alias: Thunk is shorter than Box<dyn Fn() + Send + 'static>");

    // ----- The Never Type (!) -----
    // Functions that never return have type !

    fn _bar() -> ! {
        panic!("This function never returns!");
    }

    // continue, panic!, and loop without break all have type !
    // ! can coerce to any type, which is why this works:
    let _guess: u32 = match "42".parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),  // panic! returns !
    };

    println!("Never type (!): Used for diverging functions");

    // ----- Dynamically Sized Types (DST) -----
    // Types whose size is only known at runtime: str, [T], dyn Trait
    // Must always be behind a pointer: &str, Box<[T]>, &dyn Trait

    // Sized trait is automatically bound for generic types
    // fn generic<T>(t: T) {} is really fn generic<T: Sized>(t: T) {}

    // Use ?Sized to opt out of the Sized requirement
    fn _accepts_unsized<T: ?Sized>(_t: &T) {
        // T may or may not be Sized
    }

    println!("DST: str, [T], dyn Trait must be behind a pointer");

    println!();
}

// =============================================================================
// PART 4: ADVANCED FUNCTIONS AND CLOSURES
// =============================================================================

fn advanced_functions() {
    println!("--- Part 4: Advanced Functions and Closures ---\n");

    // ----- Function Pointers -----
    // fn type (lowercase) is a function pointer

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    println!("Function pointer: do_twice(add_one, 5) = {answer}");

    // fn implements all three closure traits: Fn, FnMut, FnOnce
    // Can use function pointer where closure expected

    let list_of_numbers = vec![1, 2, 3];
    let strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())  // Closure
        .collect();
    println!("With closure: {:?}", strings);

    let strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)  // Function pointer (method)
        .collect();
    println!("With fn pointer: {:?}", strings);

    // Tuple struct initializers are also functions
    #[derive(Debug)]
    struct Status(i32);

    let list_of_statuses: Vec<Status> = (0..5).map(Status).collect();
    println!("Tuple struct as fn: {:?}", list_of_statuses);

    // ----- Returning Closures -----
    // Closures are represented by traits, so we can't return them directly
    // Use impl Trait or Box<dyn Trait>

    fn returns_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    let closure = returns_closure();
    println!("Returned closure: closure(5) = {}", closure(5));

    // If you need to return different closures conditionally,
    // you must use Box<dyn Fn>
    fn returns_boxed_closure(add: bool) -> Box<dyn Fn(i32) -> i32> {
        if add {
            Box::new(|x| x + 1)
        } else {
            Box::new(|x| x - 1)
        }
    }

    let add_closure = returns_boxed_closure(true);
    let sub_closure = returns_boxed_closure(false);
    println!("Boxed closure add: {}", add_closure(10));
    println!("Boxed closure sub: {}", sub_closure(10));

    println!();
}

// =============================================================================
// PART 5: MACROS
// =============================================================================
// Macros are metaprogramming - code that writes code
// Two types: Declarative (macro_rules!) and Procedural (derive, attribute, function-like)

fn macro_examples() {
    println!("--- Part 5: Macros ---\n");

    // ----- Declarative Macros with macro_rules! -----
    // Pattern matching on Rust code structure

    // Simple macro that prints and returns value
    macro_rules! print_result {
        ($expression:expr) => {
            {
                let val = $expression;
                println!("{} = {:?}", stringify!($expression), val);
                val
            }
        };
    }

    let x = print_result!(1 + 2);
    let y = print_result!(x * 2);
    print_result!(y);

    // Macro with multiple patterns (like match)
    macro_rules! calculate {
        (add $a:expr, $b:expr) => {
            $a + $b
        };
        (mul $a:expr, $b:expr) => {
            $a * $b
        };
    }

    println!("calculate!(add 2, 3) = {}", calculate!(add 2, 3));
    println!("calculate!(mul 2, 3) = {}", calculate!(mul 2, 3));

    // Variadic macro (like vec!)
    macro_rules! my_vec {
        // Empty case
        () => {
            Vec::new()
        };
        // One or more elements
        ($($x:expr),+ $(,)?) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )+
                temp_vec
            }
        };
    }

    let v1: Vec<i32> = my_vec!();
    let v2 = my_vec![1, 2, 3];
    let v3 = my_vec![4, 5, 6,];  // Trailing comma OK
    println!("my_vec!() = {:?}", v1);
    println!("my_vec![1, 2, 3] = {:?}", v2);
    println!("my_vec![4, 5, 6,] = {:?}", v3);

    // ----- Procedural Macros (overview) -----
    // Three kinds:
    // 1. Custom derive: #[derive(MyTrait)]
    // 2. Attribute-like: #[route(GET, "/")]
    // 3. Function-like: sql!(SELECT * FROM users)

    // Procedural macros must be in their own crate with proc-macro = true
    // They receive TokenStream input and produce TokenStream output

    println!("\nProcedural macros:");
    println!("  #[derive(Debug)] - adds Debug implementation");
    println!("  #[derive(Clone)] - adds Clone implementation");
    println!("  Custom derives require proc-macro crate");

    println!();
}

// =============================================================================
// KEY CONCEPTS SUMMARY
// =============================================================================
//
// UNSAFE RUST:
// | Operation                    | Why Unsafe?                      |
// |------------------------------|----------------------------------|
// | Dereference raw pointer      | Pointer might be invalid         |
// | Call unsafe function         | May have invariants to uphold    |
// | Modify mutable static        | Risk of data races               |
// | Implement unsafe trait       | Must maintain invariants         |
// | Access union field           | No guarantee which field is set  |
//
// ADVANCED TRAITS:
// - Associated types: Type placeholder in trait (one impl per type)
// - Default type params: Add<Rhs = Self>
// - Fully qualified: <Type as Trait>::method()
// - Supertraits: trait A: B means A requires B
// - Newtype: Wrapper type to impl foreign trait on foreign type
//
// ADVANCED TYPES:
// - Type aliases: type Name = OtherType
// - Never type (!): For functions that don't return
// - DST: Types without known size (str, [T], dyn Trait)
// - ?Sized: Relax the implicit Sized bound
//
// FUNCTIONS AND CLOSURES:
// - fn type: Function pointer, implements Fn, FnMut, FnOnce
// - Return closure: impl Fn() or Box<dyn Fn()>
//
// MACROS:
// - Declarative: macro_rules! with pattern matching
// - Procedural: Custom derive, attribute, function-like
// - Run at compile time, operate on syntax trees
// =============================================================================
