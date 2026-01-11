// =============================================================================
// CHAPTER 17: ASYNC AND AWAIT - Asynchronous Programming in Rust
// =============================================================================
// Async programming allows concurrent execution without threads.
// Key concepts:
// 1. async fn - declares an async function that returns a Future
// 2. .await   - suspends execution until Future completes
// 3. Runtime  - executes Futures (tokio, async-std, trpl)
// 4. Streams  - async iterators that yield multiple values over time
// 5. Future trait - the core abstraction behind async/await
//
// IMPORTANT: Futures are LAZY - they do nothing until awaited!
// =============================================================================

use trpl::{Either, Html, StreamExt, ReceiverStream};
use std::pin::{Pin, pin};
use std::future::Future;
use std::task::{Context, Poll};
use std::time::Duration;

// =============================================================================
// PART 1: BASIC ASYNC FUNCTIONS
// =============================================================================
// An async fn automatically returns impl Future<Output = T>
// The function body becomes the Future's poll implementation

/// Fetches a web page and extracts its title
/// Returns tuple of (url, Option<title>)
async fn page_title(url: &str) -> (String, Option<String>) {
    // .await suspends this function until get() completes
    // While waiting, the runtime can run other tasks!
    let response_text = trpl::get(url).await.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|t| t.inner_html());

    (url.to_string(), title)
}

// =============================================================================
// PART 2: SIMPLE ASYNC EXAMPLE
// =============================================================================
async fn say_hello() {
    println!("Hello");
}

async fn say_world() {
    println!("World");
}

// =============================================================================
// PART 3: ASYNC WITH DELAYS
// =============================================================================
// This helper function is used in examples below
#[allow(dead_code)]
async fn count_with_delay(name: &str, count: u32) {
    for i in 1..=count {
        println!("{name}: {i}");
        // Sleep yields control back to the runtime
        trpl::sleep(Duration::from_millis(500)).await;
    }
}

fn main() {
    // =========================================================================
    // RUNNING ASYNC CODE
    // =========================================================================
    // Async code needs a RUNTIME to execute. trpl::run creates one.
    // The runtime manages task scheduling and I/O.

    println!("=== Part 1: Basic Async Execution ===\n");

    trpl::run(async {
        // These run SEQUENTIALLY (one after another)
        say_hello().await;
        say_world().await;
    });

    // =========================================================================
    // PART 4: CONCURRENT EXECUTION WITH JOIN
    // =========================================================================
    // join! runs multiple futures concurrently and waits for ALL to complete

    println!("\n=== Part 2: Concurrent Execution with join ===\n");

    trpl::run(async {
        // Create futures (they don't run yet - futures are lazy!)
        let future1 = async {
            for i in 1..=5 {
                println!("Task A: count {i}");
                trpl::sleep(Duration::from_millis(100)).await;
            }
            "Task A done"
        };

        let future2 = async {
            for i in 1..=5 {
                println!("Task B: count {i}");
                trpl::sleep(Duration::from_millis(150)).await;
            }
            "Task B done"
        };

        // join runs both concurrently - output will be interleaved!
        let (result1, result2) = trpl::join(future1, future2).await;
        println!("{result1}, {result2}");
    });

    // =========================================================================
    // PART 5: RACING FUTURES
    // =========================================================================
    // race! returns as soon as ONE future completes (others are cancelled)

    println!("\n=== Part 3: Racing Futures ===\n");

    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(2)).await;
            "Slow finished"
        };

        let fast = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "Fast finished"
        };

        // Only the fast one completes
        let winner = match trpl::race(slow, fast).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("Winner: {winner}");
    });

    // =========================================================================
    // PART 6: ASYNC CHANNELS - Message Passing
    // =========================================================================
    // Channels work similarly to sync channels but with async send/recv

    println!("\n=== Part 4: Async Channels ===\n");

    trpl::run(async {
        // Create a channel (tx = transmitter, rx = receiver)
        let (tx, mut rx) = trpl::channel();

        // Sender task
        let sender = async move {
            let messages = vec!["Hello", "from", "async", "channel!"];
            for msg in messages {
                tx.send(msg.to_string()).unwrap();
                println!("Sent: {msg}");
                trpl::sleep(Duration::from_millis(200)).await;
            }
            // tx is dropped here, closing the channel
        };

        // Receiver task
        let receiver = async {
            // while let loops until channel is closed
            while let Some(msg) = rx.recv().await {
                println!("Received: {msg}");
            }
            println!("Channel closed");
        };

        // Run both concurrently
        trpl::join(sender, receiver).await;
    });

    // =========================================================================
    // PART 7: JOIN_ALL - Running Many Futures
    // =========================================================================
    // join_all takes a collection of futures and runs them all concurrently

    println!("\n=== Part 5: join_all with Multiple Futures ===\n");

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // Using pin! macro to pin futures to the stack
        // Pin is required for join_all because futures must not move in memory
        let sender = pin!(async move {
            for i in 1..=3 {
                tx.send(format!("Message {i}")).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let receiver = pin!(async {
            while let Some(msg) = rx.recv().await {
                println!("Got: {msg}");
            }
        });

        // Vec of pinned futures for join_all
        // dyn Future allows different future types in same collection
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![sender, receiver];
        trpl::join_all(futures).await;
    });

    // =========================================================================
    // PART 8: SPAWN_TASK - True Parallelism
    // =========================================================================
    // spawn_task creates a new task that can run on a different thread

    println!("\n=== Part 6: Spawning Tasks ===\n");

    trpl::run(async {
        // Spawn a task - it runs independently
        let handle = trpl::spawn_task(async {
            for i in 1..=3 {
                println!("Spawned task: {i}");
                trpl::sleep(Duration::from_millis(100)).await;
            }
            42 // Return value
        });

        // Main task continues immediately
        println!("Main task: spawned a task");

        // Wait for spawned task and get result
        let result = handle.await.unwrap();
        println!("Spawned task returned: {result}");
    });

    // =========================================================================
    // PART 9: STREAMS - Async Iterators
    // =========================================================================
    // Streams are the async equivalent of iterators.
    // Iterator: fn next(&mut self) -> Option<Self::Item>
    // Stream:   async fn next(&mut self) -> Option<Self::Item>
    //
    // Streams yield multiple values over time, unlike Futures which yield one.

    println!("\n=== Part 7: Streams ===\n");

    trpl::run(async {
        // ----- Creating a Stream from a Channel -----
        // ReceiverStream wraps a channel receiver to make it a Stream
        let (tx, rx) = trpl::channel();
        let rx_stream = ReceiverStream::new(rx);

        // Sender produces values over time
        let sender = async move {
            for i in 1..=5 {
                tx.send(i * 10).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        // ----- Consuming a Stream with while let -----
        // This is the most basic way to consume a stream
        let consumer = async {
            // Pin the stream so we can poll it
            let mut stream = std::pin::pin!(rx_stream);

            println!("Consuming stream with while let:");
            while let Some(value) = StreamExt::next(&mut stream).await {
                println!("  Got: {value}");
            }
            println!("  Stream ended");
        };

        trpl::join(sender, consumer).await;
    });

    // ----- Stream Combinators -----
    println!("\nStream combinators:");

    trpl::run(async {
        let (tx, rx) = trpl::channel();

        // Send some values
        for i in 1..=10 {
            tx.send(i).unwrap();
        }
        drop(tx); // Close channel so stream ends

        let stream = ReceiverStream::new(rx);

        // filter: Keep only values matching predicate (sync predicate)
        // map: Transform each value
        // take: Limit number of items
        let processed: Vec<i32> = stream
            .filter(|x| *x % 2 == 0)   // Keep evens (sync filter)
            .map(|x| x * 100)           // Multiply by 100
            .take(3)                    // Take first 3
            .collect()
            .await;

        println!("  filter(even).map(*100).take(3) = {:?}", processed);
    });

    // ----- Stream with Racing -----
    println!("\nStream with racing (timeout pattern):");

    trpl::run(async {
        let (tx, rx) = trpl::channel();

        // Slow sender - sends one value then waits too long
        let sender = async move {
            tx.send("first").unwrap();
            println!("  Sent: first");
            trpl::sleep(Duration::from_secs(5)).await; // Too slow!
            tx.send("second").unwrap();
            println!("  Sent: second");
        };

        let receiver = async {
            let stream = ReceiverStream::new(rx);
            let mut stream = std::pin::pin!(stream);

            // Get first message
            if let Some(msg) = StreamExt::next(&mut stream).await {
                println!("  Received: {msg}");
            }

            // Race: try to get second message OR timeout
            let timeout_future = async {
                trpl::sleep(Duration::from_millis(500)).await;
                println!("  Timeout waiting for next message!");
            };

            let next_msg = async {
                if let Some(msg) = StreamExt::next(&mut stream).await {
                    println!("  Received: {msg}");
                }
            };

            // Whichever finishes first wins
            trpl::race(next_msg, timeout_future).await;
        };

        // Run sender and receiver concurrently
        // Race will cancel the slow sender once receiver times out
        trpl::race(sender, receiver).await;
    });

    // =========================================================================
    // PART 10: THE FUTURE TRAIT - Under the Hood
    // =========================================================================
    // Every async fn returns something that implements Future.
    // Understanding the Future trait helps you understand how async works.
    //
    // trait Future {
    //     type Output;
    //     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    // }
    //
    // Poll has two variants:
    // - Poll::Ready(value) - The future has completed with this value
    // - Poll::Pending      - The future is not ready yet, call poll() again later

    println!("\n=== Part 8: The Future Trait ===\n");

    // ----- Manual Future Implementation -----
    // Here's a simple future that counts down before completing

    struct CountdownFuture {
        count: u32,
    }

    impl Future for CountdownFuture {
        type Output = String;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.count == 0 {
                println!("  CountdownFuture: Ready!");
                Poll::Ready("Countdown complete!".to_string())
            } else {
                println!("  CountdownFuture: {} remaining...", self.count);
                self.count -= 1;

                // IMPORTANT: We must ensure poll() is called again!
                // In a real future, you'd register with an event (I/O, timer, etc.)
                // Here we use wake_by_ref() to immediately re-poll
                cx.waker().wake_by_ref();

                Poll::Pending
            }
        }
    }

    trpl::run(async {
        let countdown = CountdownFuture { count: 3 };
        let result = countdown.await;
        println!("  Result: {result}");
    });

    // ----- Understanding Pin -----
    println!("\nUnderstanding Pin:");
    println!("  Pin<&mut T> guarantees T won't move in memory.");
    println!("  This is required because async blocks can have self-references.");
    println!("  Example of self-reference in async:");
    println!("    async {{");
    println!("        let data = vec![1, 2, 3];");
    println!("        let reference = &data[0];  // <-- points into data");
    println!("        some_async_call().await;    // suspend point!");
    println!("        println!(\"{{}}\", reference); // reference must still be valid");
    println!("    }}");
    println!("  If the async block moved in memory, 'reference' would be invalid!");
    println!("  Pin prevents this by guaranteeing the memory location is fixed.");

    // ----- How the Runtime Polls Futures -----
    println!("\nHow the async runtime works:");
    println!("  1. Runtime calls poll() on your future");
    println!("  2. If Poll::Ready(value) -> future is done, return value");
    println!("  3. If Poll::Pending -> future registered a waker");
    println!("  4. Runtime works on other tasks");
    println!("  5. When event occurs (I/O ready, timer fires), waker is called");
    println!("  6. Runtime polls the future again");
    println!("  7. Repeat until Ready");

    // =========================================================================
    // PART 11: ASYNC TRAITS (Rust 1.75+)
    // =========================================================================
    // As of Rust 1.75, you can have async functions in traits!

    println!("\n=== Part 9: Async Traits ===\n");

    trait AsyncProcessor {
        // Async method in trait - requires Rust 1.75+
        async fn process(&self, input: i32) -> i32;
    }

    struct Doubler;

    impl AsyncProcessor for Doubler {
        async fn process(&self, input: i32) -> i32 {
            // Simulate async work
            trpl::sleep(Duration::from_millis(10)).await;
            input * 2
        }
    }

    struct Squarer;

    impl AsyncProcessor for Squarer {
        async fn process(&self, input: i32) -> i32 {
            trpl::sleep(Duration::from_millis(10)).await;
            input * input
        }
    }

    trpl::run(async {
        let doubler = Doubler;
        let squarer = Squarer;

        println!("Doubler.process(5) = {}", doubler.process(5).await);
        println!("Squarer.process(5) = {}", squarer.process(5).await);
    });

    // Note: For trait objects (dyn Trait), you still need workarounds
    // like the async-trait crate or manual boxing

    // =========================================================================
    // PART 12: PRACTICAL EXAMPLE - Fetching Web Pages
    // =========================================================================
    println!("\n=== Part 10: Practical Web Fetching ===\n");

    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 3 {
        let url1 = &args[1];
        let url2 = &args[2];

        trpl::run(async {
            // Race two page fetches - first to complete wins
            let (url, title) = match trpl::race(page_title(url1), page_title(url2)).await {
                Either::Left(result) => result,
                Either::Right(result) => result,
            };

            match title {
                Some(t) => println!("First response from {url}: {t}"),
                None => println!("No title found for {url}"),
            }
        });
    } else {
        println!("Usage: cargo run <url1> <url2>");
        println!("Example: cargo run https://rust-lang.org https://crates.io");
    }
}

// =============================================================================
// KEY CONCEPTS SUMMARY
// =============================================================================
//
// CORE ASYNC CONCEPTS:
// | Concept       | Description                                    |
// |---------------|------------------------------------------------|
// | async fn      | Function that returns a Future                 |
// | .await        | Suspend until Future completes                 |
// | Future        | Lazy computation - does nothing until polled   |
// | Runtime       | Executes futures (trpl::run, tokio, etc.)      |
// | join          | Run futures concurrently, wait for ALL         |
// | race          | Run futures concurrently, return FIRST         |
// | spawn_task    | Create independent task (may use thread pool)  |
// | Pin           | Guarantees Future won't move in memory         |
// | Channel       | Async message passing between tasks            |
//
// STREAMS (Async Iterators):
// | Concept       | Description                                    |
// |---------------|------------------------------------------------|
// | Stream        | Async iterator yielding multiple values        |
// | StreamExt     | Extension trait with combinators               |
// | next()        | Get next item from stream (async)              |
// | filter()      | Keep items matching predicate                  |
// | map()         | Transform each item                            |
// | take()        | Limit number of items                          |
// | collect()     | Gather all items into collection               |
//
// THE FUTURE TRAIT:
// | Component     | Description                                    |
// |---------------|------------------------------------------------|
// | poll()        | Check if future is ready, return Poll          |
// | Poll::Ready   | Future completed with value                    |
// | Poll::Pending | Future not ready, will wake when ready         |
// | Context       | Provides the Waker to notify runtime           |
// | Waker         | Mechanism to tell runtime to poll again        |
//
// ASYNC VS THREADS:
// - Threads: OS-level, heavy, true parallelism
// - Async: User-level, lightweight, concurrent (not always parallel)
// - Use async for I/O-bound work (network, files)
// - Use threads for CPU-bound work (computation)
//
// WHEN TO USE WHAT:
// | Scenario                        | Use                              |
// |---------------------------------|----------------------------------|
// | Many concurrent I/O operations  | async/await                      |
// | CPU-intensive computation       | Threads or rayon                 |
// | Mixed I/O and CPU               | spawn_blocking in async runtime  |
// | Simple sequential code          | Regular sync code                |
// =============================================================================
