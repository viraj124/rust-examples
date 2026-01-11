// =============================================================================
// RUST ITERATORS - Lazy Sequence Processing
// =============================================================================
// Iterators are a pattern for processing sequences of items one at a time.
// Key concepts:
// 1. Lazy evaluation - nothing happens until you consume the iterator
// 2. Composable - chain multiple operations together
// 3. Zero-cost abstractions - as fast as hand-written loops
//
// The Iterator trait requires implementing one method:
//   fn next(&mut self) -> Option<Self::Item>
// =============================================================================

// Example struct to demonstrate filtering
#[derive(PartialEq, Debug)]
struct Fashion {
    style: String,
    size: u32,
}

// =============================================================================
// ITERATOR ADAPTERS AND CONSUMERS
// =============================================================================
// Adapters: Transform iterators (lazy, return new iterator)
//   - map()    : transform each element
//   - filter() : keep elements matching predicate
//   - take()   : limit to first n elements
//   - skip()   : skip first n elements
//   - zip()    : combine two iterators
//
// Consumers: Use up the iterator (eager, return a value)
//   - collect() : gather into a collection
//   - sum()     : add all elements
//   - count()   : count elements
//   - for_each(): apply function to each element
// =============================================================================

/// Filters fashions by size using iterator methods
/// - into_iter(): takes ownership of the Vec (consuming iterator)
/// - filter(): keeps only items where predicate returns true
/// - collect(): gathers results into a new Vec
fn is_correct(types: Vec<Fashion>, my_size: u32) -> Vec<Fashion> {
    types
        .into_iter()                      // Convert Vec to consuming iterator
        .filter(|x| x.size == my_size)    // Keep items matching size
        .collect()                         // Collect into new Vec
}

fn main() {
    // This crate demonstrates iterators through tests
    // Run `cargo test` to see the iterator examples in action
    println!("Iterator examples - run `cargo test` to execute tests");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterator() {
        // =====================================================================
        // ITERATOR TYPES
        // =====================================================================
        // .iter()      -> Iterator over &T (borrows)
        // .iter_mut()  -> Iterator over &mut T (mutable borrows)
        // .into_iter() -> Iterator over T (takes ownership)
        // =====================================================================

        let mut v = vec![1, 2, 3];

        // iter_mut() gives mutable references
        let mut iter = v.iter_mut();

        // next() returns Option<&mut i32>
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);  // Iterator exhausted

        // =====================================================================
        // MAP - Transform Each Element
        // =====================================================================
        // map() is LAZY - it doesn't execute until consumed!

        let v1 = vec![1, 2, 3];

        // iter() borrows, map() transforms, collect() consumes
        let result: Vec<_> = v1
            .iter()           // Iterator over &i32
            .map(|x| x * 2)   // Transform: multiply by 2
            .collect();       // Consume: gather into Vec

        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_size() {
        // =====================================================================
        // FILTER - Keep Elements Matching Predicate
        // =====================================================================

        let styles = vec![
            Fashion { style: String::from("red"), size: 10 },
            Fashion { style: String::from("blue"), size: 20 },
            Fashion { style: String::from("green"), size: 20 },
        ];

        // Filter keeps only size == 20
        let my_styles = is_correct(styles, 20);

        assert_eq!(
            my_styles,
            vec![
                Fashion { style: String::from("blue"), size: 20 },
                Fashion { style: String::from("green"), size: 20 },
            ]
        );
    }
}

// =============================================================================
// COMMON ITERATOR PATTERNS
// =============================================================================
//
// Sum all elements:
//   let sum: i32 = vec![1, 2, 3].iter().sum();
//
// Find first match:
//   let found = vec![1, 2, 3].iter().find(|&&x| x == 2);
//
// Check if any/all match:
//   let has_even = vec![1, 2, 3].iter().any(|&x| x % 2 == 0);
//   let all_positive = vec![1, 2, 3].iter().all(|&x| x > 0);
//
// Chain operations:
//   let result: Vec<_> = data
//       .iter()
//       .filter(|x| x.is_valid())
//       .map(|x| x.transform())
//       .take(10)
//       .collect();
//
// Enumerate with index:
//   for (index, value) in vec.iter().enumerate() {
//       println!("{}: {}", index, value);
//   }
// =============================================================================

// =============================================================================
// IMPLEMENTING ITERATOR FOR CUSTOM TYPES
// =============================================================================
// struct Counter {
//     count: u32,
// }
//
// impl Iterator for Counter {
//     type Item = u32;  // What type does next() return?
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count < 5 {
//             self.count += 1;
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }
// =============================================================================
