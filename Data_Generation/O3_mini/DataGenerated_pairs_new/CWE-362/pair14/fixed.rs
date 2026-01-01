////////////////////////////////////////////////////////////////////////////////////////////////////
// Fixed Example: Eliminating Race Condition via Atomic Compare-And-Swap
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Shared {
    // Using AtomicU32 for thread-safe operations.
    count: AtomicU32,
}

impl Shared {
    pub fn new() -> Self {
        Shared { count: AtomicU32::new(0) }
    }

    // The process method now uses an atomic compare_exchange to ensure that only one 
    // thread can update the count from 0 to 1, effectively removing the race condition.
    pub fn process(&self) {
        if self.count.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            // Simulated delay to mimic operational workload.
            thread::sleep(Duration::from_millis(50));
        }
    }

    pub fn value(&self) -> u32 {
        self.count.load(Ordering::SeqCst)
    }
}

fn main() {
    let shared = Arc::new(Shared::new());
    let mut handles = vec![];

    // Spawn two threads that concurrently call the process method.
    for _ in 0..2 {
        let s = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            s.process();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count: {}", shared.value());
}