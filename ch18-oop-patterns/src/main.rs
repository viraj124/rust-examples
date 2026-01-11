// =============================================================================
// CHAPTER 18: OBJECT-ORIENTED PROGRAMMING FEATURES IN RUST
// =============================================================================
// Rust isn't purely OOP, but supports many OOP concepts:
// 1. Encapsulation - pub/private, impl blocks
// 2. Inheritance - NO traditional inheritance, use traits instead
// 3. Polymorphism - Trait objects (dyn Trait)
//
// Rust favors COMPOSITION over inheritance!
// =============================================================================

fn main() {
    println!("=== Chapter 18: OOP Patterns in Rust ===\n");

    // =========================================================================
    // PART 1: ENCAPSULATION
    // =========================================================================
    encapsulation_example();

    // =========================================================================
    // PART 2: TRAIT OBJECTS FOR POLYMORPHISM
    // =========================================================================
    polymorphism_example();

    // =========================================================================
    // PART 3: STATE PATTERN
    // =========================================================================
    state_pattern_example();

    // =========================================================================
    // PART 4: STRATEGY PATTERN
    // =========================================================================
    strategy_pattern_example();
}

// =============================================================================
// PART 1: ENCAPSULATION - Hiding Implementation Details
// =============================================================================
// Use pub to expose, keep private by default
// Implementation details hidden, only interface exposed

mod encapsulation {
    /// A collection that tracks its average
    /// Internal implementation is hidden from users
    pub struct AveragedCollection {
        list: Vec<i32>,      // Private - users can't access directly
        average: f64,        // Private - computed internally
    }

    impl AveragedCollection {
        pub fn new() -> Self {
            AveragedCollection {
                list: vec![],
                average: 0.0,
            }
        }

        // Public interface
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();  // Auto-update average
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        // Private helper - users can't call this directly
        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

fn encapsulation_example() {
    println!("--- Part 1: Encapsulation ---\n");

    let mut coll = encapsulation::AveragedCollection::new();
    coll.add(10);
    coll.add(20);
    coll.add(30);
    println!("Average after adding 10, 20, 30: {}", coll.average());

    coll.remove();
    println!("Average after removing last: {}", coll.average());

    // This would fail - list is private:
    // coll.list.push(100);  // ERROR!

    println!();
}

// =============================================================================
// PART 2: POLYMORPHISM WITH TRAIT OBJECTS
// =============================================================================
// Trait objects enable runtime polymorphism using dynamic dispatch
// Syntax: Box<dyn Trait> or &dyn Trait

/// Trait defining drawable behavior
trait Draw {
    fn draw(&self);
}

// Different types implementing the same trait
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing Button: {}x{} with label '{}'",
            self.width, self.height, self.label
        );
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing SelectBox: {}x{} with {} options",
            self.width,
            self.height,
            self.options.len()
        );
    }
}

struct TextField {
    width: u32,
    placeholder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!(
            "Drawing TextField: width={} placeholder='{}'",
            self.width, self.placeholder
        );
    }
}

/// Screen holds a collection of drawable components
/// Box<dyn Draw> = trait object, allows ANY type implementing Draw
struct Screen {
    // Vec of trait objects - can hold different types!
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn new() -> Self {
        Screen { components: vec![] }
    }

    fn add(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    fn run(&self) {
        // Polymorphic call - each component's draw() is called
        for component in self.components.iter() {
            component.draw();  // Dynamic dispatch at runtime
        }
    }
}

fn polymorphism_example() {
    println!("--- Part 2: Polymorphism with Trait Objects ---\n");

    let mut screen = Screen::new();

    // Add different types to the same collection
    screen.add(Box::new(Button {
        width: 100,
        height: 50,
        label: String::from("Click Me"),
    }));

    screen.add(Box::new(SelectBox {
        width: 150,
        height: 30,
        options: vec![
            String::from("Option 1"),
            String::from("Option 2"),
            String::from("Option 3"),
        ],
    }));

    screen.add(Box::new(TextField {
        width: 200,
        placeholder: String::from("Enter text..."),
    }));

    // Draw all components - polymorphic behavior!
    screen.run();

    println!();
}

// =============================================================================
// PART 3: STATE PATTERN - Object Changes Behavior Based on State
// =============================================================================
// Classic OOP pattern implemented in Rust using trait objects

mod blog {
    /// Blog post that goes through Draft -> PendingReview -> Published
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            // Delegate to state - only Published returns content
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            // Take ownership of state, transform it, put back
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        pub fn state_name(&self) -> &str {
            self.state.as_ref().unwrap().name()
        }
    }

    // Private trait - internal implementation detail
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""  // Default: return empty string
        }
        fn name(&self) -> &str;
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self  // Can't approve a draft - stay in Draft
        }

        fn name(&self) -> &str {
            "Draft"
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self  // Already pending - stay in PendingReview
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }

        fn name(&self) -> &str {
            "PendingReview"
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self  // Already published - stay in Published
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self  // Already published
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content  // Only Published returns actual content!
        }

        fn name(&self) -> &str {
            "Published"
        }
    }
}

fn state_pattern_example() {
    println!("--- Part 3: State Pattern ---\n");

    let mut post = blog::Post::new();

    post.add_text("Hello, this is my first blog post!");
    println!("State: {}, Content: '{}'", post.state_name(), post.content());

    post.request_review();
    println!("State: {}, Content: '{}'", post.state_name(), post.content());

    post.approve();
    println!("State: {}, Content: '{}'", post.state_name(), post.content());

    println!();
}

// =============================================================================
// PART 4: STRATEGY PATTERN - Interchangeable Algorithms
// =============================================================================
// Define a family of algorithms, encapsulate each one, make them interchangeable

trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
    fn name(&self) -> &str;
}

struct BubbleSort;
impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut Vec<i32>) {
        let len = data.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }
    fn name(&self) -> &str {
        "BubbleSort"
    }
}

struct QuickSortSimple;
impl SortStrategy for QuickSortSimple {
    fn sort(&self, data: &mut Vec<i32>) {
        data.sort();  // Use built-in sort for simplicity
    }
    fn name(&self) -> &str {
        "QuickSort"
    }
}

struct Sorter {
    strategy: Box<dyn SortStrategy>,
}

impl Sorter {
    fn new(strategy: Box<dyn SortStrategy>) -> Self {
        Sorter { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn SortStrategy>) {
        self.strategy = strategy;
    }

    fn sort(&self, data: &mut Vec<i32>) {
        println!("Sorting with {}...", self.strategy.name());
        self.strategy.sort(data);
    }
}

fn strategy_pattern_example() {
    println!("--- Part 4: Strategy Pattern ---\n");

    let mut data = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Original: {:?}", data);

    let mut sorter = Sorter::new(Box::new(BubbleSort));
    sorter.sort(&mut data);
    println!("After BubbleSort: {:?}", data);

    // Change strategy at runtime
    data = vec![64, 34, 25, 12, 22, 11, 90];
    sorter.set_strategy(Box::new(QuickSortSimple));
    sorter.sort(&mut data);
    println!("After QuickSort: {:?}", data);

    println!();
}

// =============================================================================
// KEY CONCEPTS SUMMARY
// =============================================================================
//
// | OOP Concept    | Rust Approach                                |
// |----------------|----------------------------------------------|
// | Encapsulation  | pub/private, impl blocks                     |
// | Inheritance    | NOT supported - use composition + traits     |
// | Polymorphism   | Trait objects (dyn Trait) or generics        |
// | Interfaces     | Traits                                       |
// | Abstract class | Trait with default implementations           |
//
// TRAIT OBJECTS VS GENERICS:
// - Generics: Static dispatch, faster, monomorphized at compile time
// - Trait objects: Dynamic dispatch, more flexible, runtime overhead
//
// WHEN TO USE EACH:
// - Generics: When you know types at compile time
// - Trait objects: When you need a collection of different types
//                  or plugin-style architecture
// =============================================================================
