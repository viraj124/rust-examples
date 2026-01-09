// =============================================================================
// RUST CONCURRENCY - Threads, Channels, and Shared State
// =============================================================================
// This module demonstrates Rust's fearless concurrency features:
// 1. Spawning threads with thread::spawn
// 2. Message passing with channels (mpsc)
// 3. Shared state with Mutex and Arc
// =============================================================================

use std::thread;
use std::time::Duration;
use std::sync::mpsc;         // mpsc = "multiple producer, single consumer"
use std::sync::{Arc, Mutex}; // Arc = Atomic Reference Counting (thread-safe Rc)

fn main() {
    // =========================================================================
    // PART 1: Basic Thread Spawning
    // =========================================================================
    // thread::spawn takes a closure and runs it in a new thread.
    // The closure uses `||` syntax (no arguments in this case).
    // The spawned thread runs concurrently with the main thread.

    let t1 = thread::spawn(|| {
        for _i in 0..10 {
            println!("Hello from thread 1");
            // Sleep to simulate work and allow thread interleaving
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread also does work - notice how output interleaves!
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // =========================================================================
    // join() - Wait for thread to finish
    // =========================================================================
    // Without join(), main thread might exit before spawned thread finishes.
    // join() blocks the current thread until the spawned thread completes.
    // unwrap() handles the Result - thread might panic.
    t1.join().unwrap();

    // =========================================================================
    // PART 2: Moving Data into Threads with `move`
    // =========================================================================
    // Problem: Spawned threads might outlive the data they reference.
    // Solution: Use `move` keyword to transfer ownership to the thread.

    let v = vec![1, 2, 3];

    // `move` keyword forces closure to take ownership of `v`
    // Without `move`, Rust would try to borrow `v`, but the thread might
    // outlive the current scope, creating a dangling reference!
    let handle = thread::spawn(move || {
        // v is now OWNED by this closure - it was moved here
        println!("Here's a vector: {v:?}");
    });

    // This would cause a compile error - v was moved!
    // println!("Here's a vector: {v:?}");

    handle.join().unwrap();

    // =========================================================================
    // PART 3: Message Passing with Channels
    // =========================================================================
    // Channels provide a way to send data between threads.
    // "Do not communicate by sharing memory; share memory by communicating."
    //
    // mpsc::channel() returns a tuple: (transmitter, receiver)
    // - tx (transmitter): can be cloned for multiple producers
    // - rx (receiver): only one receiver allowed

    let (tx, rx) = mpsc::channel();

    // Clone tx BEFORE moving original tx into a thread
    // This allows multiple threads to send to the same receiver
    let tx1 = tx.clone();

    // Spawn a thread that sends a single value
    thread::spawn(move || {
        // send() returns Result - Err if receiver is dropped
        tx.send(1).unwrap();
        // Note: After send(), the value is moved - can't use it anymore!
    });

    // recv() blocks until a value is received
    // try_recv() is non-blocking - returns immediately with Result
    let val = rx.recv().unwrap();
    println!("Got: {}", val);

    // =========================================================================
    // Multiple Messages from Multiple Producers
    // =========================================================================
    let data = vec![1, 2, 3];

    // This thread uses the cloned transmitter (tx1)
    thread::spawn(move || {
        for val in data {
            tx1.send(val).unwrap();
        }
        // tx1 is dropped here, but tx was already dropped above
        // When ALL transmitters are dropped, the channel closes
    });

    // Treat receiver as an iterator - loops until channel closes
    // Channel closes when all transmitters are dropped
    for recvd in rx {
        println!("Got: {}", recvd);
    }

    // =========================================================================
    // PART 4: Shared State with Mutex
    // =========================================================================
    // Mutex = Mutual Exclusion - only one thread can access data at a time
    //
    // To access data:
    // 1. Call lock() to acquire the mutex
    // 2. Returns a MutexGuard smart pointer
    // 3. MutexGuard auto-releases lock when it goes out of scope

    let m = Mutex::new(23);
    {
        // lock() returns LockResult<MutexGuard>
        // MutexGuard implements Deref, so we can use it like a reference
        let mut num = m.lock().unwrap();
        *num += 1;  // Dereference to modify the inner value
    } // MutexGuard dropped here, lock is released

    println!("{:?}", m);  // Mutex { data: 24, ... }

    // =========================================================================
    // PART 5: Sharing Mutex Between Threads with Arc
    // =========================================================================
    // Problem: Rc<T> is NOT thread-safe (no atomic operations)
    // Solution: Arc<T> = Atomic Reference Counting
    //
    // Arc has the same API as Rc, but uses atomic operations
    // for thread-safe reference counting (slight performance cost)

    // Arc<Mutex<T>> is a common pattern for shared mutable state
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Clone the Arc (increments reference count atomically)
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Each thread:
            // 1. Acquires the lock
            // 2. Increments the counter
            // 3. Releases the lock (when MutexGuard is dropped)
            *counter.lock().unwrap() += 1;
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // All threads have finished - counter should be 10
    println!("Result: {}", *counter.lock().unwrap());

    // =========================================================================
    // KEY TAKEAWAYS:
    // =========================================================================
    // 1. Use `move` to transfer ownership to spawned threads
    // 2. Use channels (mpsc) for message passing between threads
    // 3. Use Mutex for shared mutable state (one accessor at a time)
    // 4. Use Arc (not Rc) when sharing ownership across threads
    // 5. Arc<Mutex<T>> is the common pattern for shared mutable state
    // 6. Rust's ownership system prevents data races at compile time!
    // =========================================================================
}
