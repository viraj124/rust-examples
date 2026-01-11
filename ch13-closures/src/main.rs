// =============================================================================
// RUST CLOSURES - Anonymous Functions that Capture Environment
// =============================================================================
// Closures are anonymous functions that can:
// 1. Be stored in variables or passed as arguments
// 2. Capture values from their enclosing scope
// 3. Have types inferred by the compiler
//
// Closure syntax: |params| -> return_type { body }
// Short form:     |params| expression
// =============================================================================

use std::thread;
use std::time::Duration;

// =============================================================================
// Example: Using closures with Option::unwrap_or_else
// =============================================================================
// This example shows closures used for lazy evaluation - the closure only
// runs if the Option is None.

enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // unwrap_or_else takes a closure that provides a default value
    // The closure captures `self` from the environment
    fn giveaway(&self, preference: Option<ShirtColor>) -> ShirtColor {
        // If preference is Some, return it
        // If preference is None, call the closure to get default
        // The closure `|| self.most_stocked()` captures `self` by reference
        preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut color_red = 0;
        let mut color_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => color_red += 1,
                ShirtColor::Blue => color_blue += 1,
            }
        }
        if color_red > color_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red],
    };

    // None provided - closure runs to determine default
    inventory.giveaway(None);

    // Some provided - closure never runs (lazy evaluation)
    inventory.giveaway(Some(ShirtColor::Red));

    // =========================================================================
    // Closure Syntax Examples
    // =========================================================================
    // Full syntax with type annotations:
    // let closure = |x: i32| -> i32 { x + 1 };
    //
    // Shortened - types inferred:
    // let closure = |x| { x + 1 };
    //
    // Even shorter - single expression, no braces needed:
    // let closure = |x| x + 1;

    // This closure takes i32, returns i32, and sleeps (simulating expensive work)
    let executor_closure = |x| -> i32 {
        thread::sleep(Duration::from_secs(1));
        x + 1
    };

    // =========================================================================
    // Memoization Pattern with Closures
    // =========================================================================
    // Store a closure and cache its result to avoid repeated expensive calls

    let mut executor_info = Executor::new(executor_closure);

    // First call: closure runs, result cached (takes ~1 second)
    println!("{}", executor_info.value(3));  // Prints: 4

    // Second call: returns cached value immediately (no wait)
    // Note: This simple cache ignores the argument! See improvement below.
    println!("{}", executor_info.value(4));  // Prints: 4 (cached, not 5!)
}

// =============================================================================
// MEMOIZATION STRUCT - Caching Expensive Closure Results
// =============================================================================
// This pattern stores a closure and caches its result.
//
// CLOSURE TRAITS:
// - Fn:     borrows values immutably (can be called multiple times)
// - FnMut:  borrows values mutably (can be called multiple times)
// - FnOnce: takes ownership (can only be called once)
//
// All closures implement FnOnce. Most also implement FnMut and Fn.
// =============================================================================

struct Executor<T>
where
    T: Fn(i32) -> i32,  // T must be a closure/function: i32 -> i32
{
    cache: T,            // The closure itself
    value: Option<i32>,  // Cached result (None if not computed yet)
}

impl<T> Executor<T>
where
    T: Fn(i32) -> i32,
{
    // Constructor - stores closure, no cached value yet
    fn new(cache: T) -> Executor<T> {
        Executor {
            cache,
            value: None,
        }
    }

    // Get value - compute once, then return cached result
    fn value(&mut self, x: i32) -> i32 {
        match self.value {
            // Already computed - return cached value
            Some(val) => val,
            // Not computed yet - run closure and cache result
            None => {
                // (self.cache)(x) calls the stored closure
                let val = (self.cache)(x);
                self.value = Some(val);
                val
            }
        }
    }
}

// =============================================================================
// CLOSURE CAPTURE MODES
// =============================================================================
// Closures can capture variables from their environment in three ways:
//
// 1. By reference (&T) - borrows immutably
//    let x = 5;
//    let closure = || println!("{}", x);  // x borrowed
//
// 2. By mutable reference (&mut T) - borrows mutably
//    let mut x = 5;
//    let mut closure = || x += 1;  // x mutably borrowed
//
// 3. By value (T) - takes ownership (use `move` keyword)
//    let x = vec![1, 2, 3];
//    let closure = move || println!("{:?}", x);  // x moved into closure
//
// The compiler chooses the least restrictive capture mode that works.
// Use `move` to force ownership transfer (needed for threads).
// =============================================================================

// =============================================================================
// IMPROVEMENT IDEAS FOR LEARNING:
// =============================================================================
// The current Executor has a bug - it ignores the argument after first call!
//
// Better implementation would use HashMap<i32, i32> to cache per-argument:
//
// struct BetterExecutor<T>
// where T: Fn(i32) -> i32,
// {
//     cache: T,
//     values: HashMap<i32, i32>,  // Map argument -> result
// }
//
// impl<T> BetterExecutor<T>
// where T: Fn(i32) -> i32,
// {
//     fn value(&mut self, x: i32) -> i32 {
//         *self.values.entry(x).or_insert_with(|| (self.cache)(x))
//     }
// }
// =============================================================================
