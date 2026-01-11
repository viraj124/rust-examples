// =============================================================================
// RUST GENERICS - Write Code That Works with Multiple Types
// =============================================================================
// Generics allow you to write flexible, reusable code that works with any type.
// They provide:
// 1. Code reuse - write once, use with many types
// 2. Type safety - compiler checks types at compile time
// 3. Zero runtime cost - monomorphization creates specialized code
//
// Syntax: <T> where T is a type parameter (can be any name, T is convention)
// =============================================================================

// =============================================================================
// GENERIC STRUCTS
// =============================================================================
// Structs can have generic type parameters

// Single generic parameter - x and y must be SAME type
struct Point<T> {
    x: T,
    y: T,
}

// Two generic parameters - x and y can be DIFFERENT types
struct DoublePoint<T, U> {
    x: T,
    y: U,
}

// =============================================================================
// IMPLEMENTING METHODS ON GENERIC STRUCTS
// =============================================================================

// impl<T: Copy> means "implement for any type T that implements Copy trait"
// The Copy bound is needed because we return self.x by value
impl<T: Copy> Point<T> {
    fn get_x(&self) -> T {
        self.x  // Returns a copy of x
    }
}

// Concrete implementation - only for Point<f64>
// This method is ONLY available when T = f64
impl Point<f64> {
    fn get_x_f64(&self) -> f64 {
        self.x
    }
}

// Generic impl with multiple type parameters
// T: Copy means T must implement Copy (so we can copy self.x)
impl<T: Copy, U> DoublePoint<T, U> {
    // Mix types from self and another DoublePoint
    // V and W are NEW generic parameters just for this method
    fn mix_up<V, W>(&self, other: DoublePoint<V, W>) -> DoublePoint<T, W> {
        DoublePoint {
            x: self.x,   // From self (type T)
            y: other.y,  // From other (type W)
        }
    }
}

fn main() {
    // =========================================================================
    // Using Generic Functions
    // =========================================================================

    let int_val = vec![1, 2, 3, 4, 5];
    let _large_int = get_largest(int_val);  // T inferred as i32

    let char_val = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let _large_char = get_largest(char_val);  // T inferred as char

    // =========================================================================
    // Generic Structs with Same Type
    // =========================================================================

    let _p1 = Point { x: 5, y: 99 };     // Point<i32>
    let _p2 = Point { x: 1.5, y: 2.4 };  // Point<f64>

    // This would NOT compile - x and y must be same type:
    // let p3 = Point { x: 5, y: 2.4 };  // Error! i32 != f64

    // =========================================================================
    // Generic Structs with Different Types
    // =========================================================================

    let dp_1 = DoublePoint { x: 5, y: 2.4 };    // DoublePoint<i32, f64>
    let dp_2 = DoublePoint { x: 4, y: 200 };    // DoublePoint<i32, i32>

    // mix_up creates DoublePoint<i32, i32> (x from dp_1, y from dp_2)
    let _new_dp = dp_1.mix_up(dp_2);
}

// =============================================================================
// GENERIC FUNCTIONS WITH TRAIT BOUNDS
// =============================================================================
// Problem: Not all types support comparison (>)
// Solution: Use trait bounds to constrain which types are allowed
//
// T: PartialOrd  - T must support comparison (<, >, ==)
// T: Copy        - T must be copyable (so we can copy from vec without move)
//
// Syntax options:
//   fn example<T: Trait1 + Trait2>(x: T)           // Inline bounds
//   fn example<T>(x: T) where T: Trait1 + Trait2   // Where clause (cleaner)
// =============================================================================

fn get_largest<T: PartialOrd + Copy>(var_list: Vec<T>) -> T {
    // Start with first element (Copy trait lets us copy it out)
    let mut largest: T = var_list[0];

    // Compare each element
    for num in var_list {
        // PartialOrd trait enables the > operator
        if num > largest {
            largest = num;
        }
    }
    largest
}

// =============================================================================
// MONOMORPHIZATION - Zero-Cost Generics
// =============================================================================
// At compile time, Rust generates specialized versions for each concrete type:
//
// get_largest::<i32>()   - version for integers
// get_largest::<char>()  - version for characters
//
// This means generics have NO runtime performance cost!
// The compiled code is identical to if you wrote separate functions.
// =============================================================================

// =============================================================================
// COMMON GENERIC PATTERNS
// =============================================================================
//
// Option<T> - Maybe a value:
//   enum Option<T> { Some(T), None }
//
// Result<T, E> - Success or error:
//   enum Result<T, E> { Ok(T), Err(E) }
//
// Vec<T> - Dynamic array:
//   struct Vec<T> { ... }
//
// HashMap<K, V> - Key-value store:
//   struct HashMap<K, V> { ... }
// =============================================================================
