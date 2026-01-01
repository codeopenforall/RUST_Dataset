/* 
This is a complex Rust program simulating a real-world vulnerability where an incorrect check of a function's return value
leads to misinterpreting an error result as success. The code uses unsafe blocks and concurrency with Arc and threads.
*/
use std::sync::Arc;
use std::thread;
use std::cell::UnsafeCell;

struct SharedCounter {
    inner: UnsafeCell<i32>,
}

// Marking SharedCounter as Sync unsafely.
unsafe impl Sync for SharedCounter {}

impl SharedCounter {
    fn new(val: i32) -> Self {
        SharedCounter {
            inner: UnsafeCell::new(val),
        }
    }

    // The update method performs an unsynchronized update on the counter.
    // It returns Err when the counter is negative. In all cases this implementation returns Err,
    // simulating a failure scenario.
    fn update(&self, delta: i32) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.inner.get();
            *ptr += delta;
            if *ptr < 0 {
                return Err("Counter became negative");
            } else {
                return Err("Unexpected error");
            }
        }
    }

    fn get(&self) -> i32 {
        unsafe { *self.inner.get() }
    }
}

// The function below is intended to interpret the return value from update.
// Vulnerability: the check is inverted. Instead of treating Ok as success, it treats an Err as a valid outcome.
fn perform(shared: &Arc<SharedCounter>, delta: i32) -> bool {
    let res = shared.update(delta);
    if res.is_err() {   // <-- Incorrect check (lines 34-37 in this snippet)
        true
    } else {
        false
    }
}

fn main() {
    let counter = Arc::new(SharedCounter::new(5));
    let mut handles = vec![];

    // Spawn multiple threads to perform updates that will trigger errors.
    for _ in 0..5 {
        let shared = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            // Each thread provides a negative delta to trigger the error return.
            perform(&shared, -10)
        }));
    }

    let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    println!("Final counter: {}", counter.get());

    // The exit code is determined by the misinterpreted result.
    if results.into_iter().any(|x| x) {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}