/////////////////// Fixed Version ///////////////////
// This version replaces the unsynchronized UnsafeCell with an AtomicU32,
// ensuring thread-safe operations with proper memory ordering.

use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

static GLOBAL: AtomicU32 = AtomicU32::new(0);

fn update(count: u32) {
    for _ in 0..count {
        // Atomically increment the counter.
        GLOBAL.fetch_add(1, Ordering::SeqCst);
    }
}

fn run() -> u32 {
    let mut handles = Vec::new();
    // Spawn multiple threads that concurrently update the global counter safely.
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            update(1000);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // Return the final counter value.
    GLOBAL.load(Ordering::SeqCst)
}

fn main() {
    let result = run();
    println!("Final count: {}", result);
}