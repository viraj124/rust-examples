// =============================================================================
// RUST TRAITS - Shared Behavior Across Types
// =============================================================================
// Traits define shared behavior (like interfaces in other languages).
// They allow:
// 1. Defining shared methods that types can implement
// 2. Constraining generic types (trait bounds)
// 3. Returning types that implement a trait (impl Trait)
// 4. Default implementations that can be overridden
// =============================================================================

use std::fmt::Display;

fn main() {
    let news = NewsArticle {
        headline: String::from("Breaking News!"),
        location: String::from("New York"),
        author: String::from("Jane Doe"),
        content: String::from("Something important happened..."),
    };

    let _tweet = Tweet {
        username: String::from("rustacean"),
        retweet: true,
        reply: true,
        content: String::from("Rust is awesome!"),
    };

    println!("Hello, world!");

    // Using trait method through function that accepts impl Summary
    trait_impl(&news);

    // Using impl Trait return type
    println!("{}", returns_trait_struct().summarize());
}

// =============================================================================
// DEFINING STRUCTS
// =============================================================================

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Generic struct with trait bounds on implementation
struct Pair<T> {
    x: T,
    y: T,
}

// =============================================================================
// CONDITIONAL IMPLEMENTATION WITH TRAIT BOUNDS
// =============================================================================
// This impl block only applies to Pair<T> where T: Display + PartialOrd
// The method cmp_display is only available when T meets these bounds

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        }
        println!("The largest member is y = {}", self.y)
    }
}

// =============================================================================
// DEFINING TRAITS
// =============================================================================

// Trait with a default implementation
// Types can use the default or override it
pub trait Summary {
    // Default implementation - types get this for free
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Trait requiring implementation (no default)
pub trait Article {
    fn summary(&self) -> String;  // No body = must implement
}

// =============================================================================
// SUPERTRAITS - Trait that requires another trait
// =============================================================================
// Debug: std::fmt::Debug means any type implementing our Debug trait
// must ALSO implement std::fmt::Debug (the standard library trait)

pub trait Debug: std::fmt::Debug {
    fn debug(&self) {
        // This works because we know self implements std::fmt::Debug
        println!("{:?}", self);
    }
}

// =============================================================================
// IMPLEMENTING TRAITS FOR TYPES
// =============================================================================

// NewsArticle overrides the default summarize()
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("(Read {} from {}... by {})", self.headline, self.location, self.author)
    }
}

// Tweet also overrides summarize()
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Article for NewsArticle {
    fn summary(&self) -> String {
        format!("{} of {} by {}", self.headline, self.location, self.author)
    }
}

// =============================================================================
// TRAITS AS FUNCTION PARAMETERS
// =============================================================================

// Method 1: impl Trait syntax (simple, recommended for simple cases)
// "Accept any type that implements Summary"
fn trait_impl(val: &impl Summary) {
    println!("{}", val.summarize());
}

// Method 2: Trait bound syntax (more flexible)
// Equivalent to above, but allows more complex constraints
fn trait_bound_impl<T: Summary>(val: &T) {
    println!("{}", val.summarize());
}

// Method 3: Where clause (cleaner for multiple bounds)
// T must implement Summary AND Article
// U must implement Article AND Summary
fn trait_bound_impl_complex<T, U>(item1: &T, item2: &U)
where
    T: Summary + Article,
    U: Article + Summary,
{
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

// =============================================================================
// RETURNING TYPES THAT IMPLEMENT A TRAIT
// =============================================================================
// impl Summary as return type means "returns some type that implements Summary"
// Useful for returning closures or hiding concrete types
//
// LIMITATION: Can only return ONE concrete type!
// This won't compile:
// fn returns_trait() -> impl Summary {
//     if condition { Tweet { ... } }      // Error! Can't return
//     else { NewsArticle { ... } }        // different types
// }

fn returns_trait_struct() -> impl Summary {
    // Can only return Tweet here, not NewsArticle
    Tweet {
        username: String::from("dd"),
        retweet: true,
        reply: true,
        content: String::from("dd"),
    }
}

// =============================================================================
// TRAIT BOUNDS SYNTAX COMPARISON
// =============================================================================
//
// Simple bound:
//   fn example(val: &impl Summary)
//   fn example<T: Summary>(val: &T)
//
// Multiple traits (both required):
//   fn example(val: &(impl Summary + Display))
//   fn example<T: Summary + Display>(val: &T)
//
// Where clause (cleaner for complex bounds):
//   fn example<T, U>(x: &T, y: &U)
//   where
//       T: Summary + Display,
//       U: Clone + Debug
//   { ... }
//
// Conditional implementation:
//   impl<T: Display + PartialOrd> Pair<T> { ... }
// =============================================================================

// =============================================================================
// BLANKET IMPLEMENTATIONS
// =============================================================================
// Standard library uses this pattern extensively:
//
// impl<T: Display> ToString for T {
//     // Any type that implements Display automatically gets ToString
// }
//
// This is why you can call .to_string() on any Display type!
// =============================================================================
