////////////////////////////////////////////////////////////
// Corrected Code for Race Condition (CWE-362)
// This version uses an atomic counter to ensure that updates from multiple 
// threads are performed atomically, removing the race condition.
////////////////////////////////////////////////////////////
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

// Use an atomic counter instead of an unsafe mutable static variable.
static GLOBAL: AtomicU32 = AtomicU32::new(0);

fn run() -> u32 {
    // Reset the counter atomically.
    GLOBAL.store(0, Ordering::SeqCst);

    let mut handles = vec![];
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            for _ in 0..1000 {
                // Atomic increment ensures no data races occur.
                GLOBAL.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    GLOBAL.load(Ordering::SeqCst)
}

fn main() {
    let result = run();
    println!("Final counter value: {}", result);
}