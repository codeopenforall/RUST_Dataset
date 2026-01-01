////////////////////////////////////////////////////////////////////////////////////////////////////
// CWE-362 Example: Race Condition via Unsynchronized Check-Then-Act Using UnsafeCell
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Shared {
    // UnsafeCell permits mutable access without synchronization.
    count: UnsafeCell<u32>,
}

// Marking Shared as Sync unsafely to allow shared mutable access.
unsafe impl Sync for Shared {}

impl Shared {
    pub fn new() -> Self {
        Shared { count: UnsafeCell::new(0) }
    }

    // Vulnerable function: performs an unsynchronized check-then-set operation.
    // If multiple threads enter the critical section concurrently, this can 
    // result in multiple increments (a classic race condition).
    pub fn process(&self) {
        unsafe {
            // Check if count is zero.
            if *self.count.get() == 0 {
                // Artificial delay to widen the race window.
                thread::sleep(Duration::from_millis(50));
                // Unsynchronized update: race condition may allow multiple threads here.
                *self.count.get() += 1;
            }
        }
    }

    pub fn value(&self) -> u32 {
        unsafe { *self.count.get() }
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