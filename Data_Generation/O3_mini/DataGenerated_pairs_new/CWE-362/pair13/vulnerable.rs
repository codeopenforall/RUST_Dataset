/////////////////// Vulnerable Version ///////////////////
// This version uses an unsynchronized global mutable counter through UnsafeCell,
// leading to race conditions when incremented concurrently.

use std::cell::UnsafeCell;
use std::thread;

static mut GLOBAL: UnsafeCell<u32> = UnsafeCell::new(0);

fn update(count: u32) {
    for _ in 0..count {
        // Unsafe unsynchronized access introduces a race condition.
        unsafe {
            let ptr = GLOBAL.get();
            let current = *ptr;
            // Yield to magnify interleaving effects.
            thread::yield_now();
            *ptr = current.wrapping_add(1);
        }
    }
}

fn run() -> u32 {
    let mut handles = Vec::new();
    // Spawn multiple threads that concurrently update the global counter.
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            update(1000);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // Return the final counter value.
    unsafe { *GLOBAL.get() }
}

fn main() {
    let result = run();
    println!("Final count: {}", result);
}