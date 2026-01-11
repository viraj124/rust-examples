// =============================================================================
// RUST SMART POINTERS - Memory Management Beyond References
// =============================================================================
// Smart pointers are data structures that act like pointers but have additional
// metadata and capabilities. They implement Deref and Drop traits.
//
// Key smart pointers covered:
// 1. Box<T>     - Heap allocation with single ownership
// 2. Rc<T>      - Reference counting for multiple ownership
// 3. RefCell<T> - Interior mutability (runtime borrow checking)
// 4. Weak<T>    - Non-owning reference to prevent cycles
// =============================================================================

mod lib;  // Reference cycle demonstration in lib.rs
use crate::lib::List::{Cons, Nil};

use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

// =============================================================================
// TRAIT FOR DEMONSTRATING INTERIOR MUTABILITY
// =============================================================================
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct Email<'a, T: Messenger> {
    messenger: &'a T,
    max: usize,
    value: usize,
}

fn main() {
    // =========================================================================
    // PART 1: DEREFERENCING - Following Pointers to Values
    // =========================================================================

    let x = 5;
    let y = &x;  // y is a reference to x

    // Dereference with * to get the value
    println!("{}", *y);   // Prints: 5
    println!("{}", x);    // Prints: 5

    assert_eq!(5, x);
    // This won't compile - can't compare i32 with &i32:
    // assert_eq!(5, y);  // Error! Use *y instead

    // =========================================================================
    // PART 2: Box<T> - Heap Allocation
    // =========================================================================
    // Box<T> stores data on the heap instead of the stack.
    // Use cases:
    // - Recursive types (type size unknown at compile time)
    // - Large data you want to transfer ownership without copying
    // - Trait objects (dyn Trait)

    let z = Box::new(x);
    // Box implements Deref, so * works just like with references
    assert_eq!(5, *z);

    // =========================================================================
    // PART 3: Custom Smart Pointer with Deref
    // =========================================================================
    // Implementing Deref lets you use * on your type

    let w = MyBox::new(6);
    assert_eq!(6, *w);  // Works because MyBox implements Deref!

    // =========================================================================
    // PART 4: Deref Coercion
    // =========================================================================
    // Rust automatically converts &T to &U if T: Deref<Target=U>
    // This enables passing &Box<String> where &str is expected

    let name = Box::new(String::from("Viraz Malhotra"));
    // Without deref coercion, we'd need: &(*name)[..]
    // With deref coercion, this works: &name or &*name
    println!("{}", &(*name)[..]);

    // =========================================================================
    // PART 5: Drop Trait - Cleanup When Value Goes Out of Scope
    // =========================================================================
    // Drop is called automatically when a value goes out of scope.
    // Use it to release resources (files, network connections, etc.)

    let drop_ex = DropExample { name: String::from("Viraz Malhotra") };
    println!("drop starts");

    // Force early drop with std::mem::drop()
    // (Can't call drop_ex.drop() directly - Rust prevents that)
    drop(drop_ex);
    // drop_ex can't be used after this!

    // =========================================================================
    // PART 6: Rc<T> - Reference Counted Pointer (Multiple Ownership)
    // =========================================================================
    // Rc<T> enables multiple parts of your program to own the same data.
    // The data is cleaned up when the last Rc is dropped.
    //
    // NOTE: Rc is NOT thread-safe! Use Arc for threads.

    // Uncomment to see Rc in action:
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("Count after creating a: {}", Rc::strong_count(&a));  // 1
    // let b = Cons(7, Rc::clone(&a));  // Clone increments count
    // println!("Count after creating b: {}", Rc::strong_count(&a));  // 2
    // {
    //     let c = Cons(8, Rc::clone(&a));
    //     println!("Count after creating c: {}", Rc::strong_count(&a));  // 3
    // }
    // println!("Count after c goes out of scope: {}", Rc::strong_count(&a));  // 2

    // =========================================================================
    // PART 7: RefCell<T> - Interior Mutability
    // =========================================================================
    // Problem: Rust's borrowing rules are checked at COMPILE time.
    // Sometimes you need to mutate data even when you have immutable references.
    //
    // Solution: RefCell<T> moves borrow checking to RUNTIME.
    // - borrow()     -> immutable reference (&T)
    // - borrow_mut() -> mutable reference (&mut T)
    //
    // PANICS at runtime if you violate borrowing rules!

    // Interior mutability example - Email tracker
    impl<'a, T> Email<'a, T>
    where
        T: Messenger,
    {
        fn new(messenger: &'a T, max: usize) -> Self {
            Email {
                messenger,
                max,
                value: 0,
            }
        }

        fn set_value(&mut self, value: usize) {
            self.value = value;
            let used_quota = self.value as f64 / self.max as f64;

            if used_quota > 1.0 {
                self.messenger.send("Email quota exceeded");
            } else if used_quota == 0.9 {
                self.messenger.send("Email quota at 90%");
            } else {
                self.messenger.send("Email quota is low");
            }
        }
    }

    // =========================================================================
    // PART 8: Combining Rc<T> and RefCell<T>
    // =========================================================================
    // Rc<RefCell<T>> gives you:
    // - Multiple owners (Rc)
    // - Mutable data (RefCell)

    let value = Rc::new(RefCell::new(5));

    // Create lists that share ownership of value
    let list1 = Rc::new(New_Cons(Rc::clone(&value), Rc::new(New_Nil)));
    let list2 = New_Cons(Rc::new(RefCell::new(4)), Rc::clone(&list1));
    let list3 = New_Cons(Rc::new(RefCell::new(7)), Rc::clone(&list1));

    // Mutate the shared value through RefCell
    *value.borrow_mut() = 10;

    // Can also mutate through list2's head
    if let New_Cons(head, _) = &list2 {
        *head.borrow_mut() = 10;
    }

    println!("{:?}", list1);
    println!("{:?}", list2);
    println!("{:?}", list3);

    // =========================================================================
    // PART 9: Reference Cycles and Memory Leaks
    // =========================================================================
    // Rc can create cycles that never get cleaned up (memory leak!).
    // Solution: Use Weak<T> for non-owning references.

    let ref_1 = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("ref_1 initial rc count = {}", Rc::strong_count(&ref_1));
    println!("ref_1 next item = {:?}", ref_1.tail());

    let ref_2 = Rc::new(Cons(10, RefCell::new(Rc::clone(&ref_1))));
    println!("ref_1 count after ref_2 creation = {}", Rc::strong_count(&ref_1));
    println!("ref_2 initial count = {}", Rc::strong_count(&ref_2));
    println!("ref_2 next item = {:?}", ref_2.tail());

    // Creating a cycle! ref_1 -> ref_2 -> ref_1 -> ...
    if let Some(link) = ref_1.tail() {
        *link.borrow_mut() = Rc::clone(&ref_2);
    }

    println!("ref_2 count after cycle = {}", Rc::strong_count(&ref_2));
    println!("ref_1 count after cycle = {}", Rc::strong_count(&ref_1));

    // WARNING: This would cause stack overflow due to infinite cycle:
    // println!("ref_1 next item = {:?}", ref_1.tail());
}

// =============================================================================
// TESTS FOR INTERIOR MUTABILITY
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Mock messenger that tracks messages using RefCell for interior mutability
    struct MockMessenger {
        msgs: RefCell<Vec<String>>,  // RefCell allows mutation through &self
    }

    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                msgs: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // &self is immutable, but we can mutate msgs through RefCell!
            self.msgs.borrow_mut().push(msg.to_string());

            // WARNING: This would PANIC at runtime - two mutable borrows!
            // let mut one_borrow = self.msgs.borrow_mut();
            // let mut two_borrow = self.msgs.borrow_mut();  // PANIC!
        }
    }

    #[test]
    fn test_send_with_90_quota() {
        let mock = MockMessenger::new();
        let mut tracker = Email::new(&mock, 100);
        tracker.set_value(90);
        assert_eq!(mock.msgs.borrow().len(), 1);
    }
}

// =============================================================================
// ENUM FOR Rc<RefCell<T>> DEMONSTRATION
// =============================================================================
use New_List::{Cons as New_Cons, Nil as New_Nil};

#[derive(Debug)]
enum New_List {
    Cons(Rc<RefCell<i32>>, Rc<New_List>),
    Nil,
}

// =============================================================================
// CUSTOM SMART POINTER - Implementing Deref
// =============================================================================
// Tuple struct wrapping a single value
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> Self {
        MyBox(t)
    }
}

// Implement Deref to enable * operator
impl<T> Deref for MyBox<T> {
    type Target = T;  // Associated type - what we dereference to

    fn deref(&self) -> &Self::Target {
        &self.0  // Return reference to inner value
    }
}
// When you write *y, Rust actually runs: *(y.deref())

// =============================================================================
// CUSTOM DROP IMPLEMENTATION
// =============================================================================
struct DropExample {
    name: String,
}

impl Drop for DropExample {
    fn drop(&mut self) {
        // Called automatically when value goes out of scope
        // Use for cleanup: closing files, releasing locks, etc.
        self.name.push_str(" - Dropped");
        println!("Dropping: {}", self.name);
    }
}

// =============================================================================
// SMART POINTER SUMMARY
// =============================================================================
//
// | Type         | Ownership    | Borrow Check | Thread Safe | Use Case                    |
// |--------------|--------------|--------------|-------------|------------------------------|
// | Box<T>       | Single       | Compile time | Yes*        | Heap allocation, recursion   |
// | Rc<T>        | Multiple     | Compile time | NO          | Shared ownership (single)    |
// | Arc<T>       | Multiple     | Compile time | YES         | Shared ownership (threads)   |
// | RefCell<T>   | Single       | Runtime      | NO          | Interior mutability          |
// | Mutex<T>     | Single       | Runtime      | YES         | Interior mutability (threads)|
// | Weak<T>      | Non-owning   | N/A          | NO          | Breaking cycles              |
//
// Common Combinations:
// - Rc<RefCell<T>>  : Multiple owners with mutation (single-threaded)
// - Arc<Mutex<T>>   : Multiple owners with mutation (multi-threaded)
// =============================================================================
