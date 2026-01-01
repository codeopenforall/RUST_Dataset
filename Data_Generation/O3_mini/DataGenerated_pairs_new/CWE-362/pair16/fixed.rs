//////////////////////////////////////////////
// Fixed Version – Race Condition Fixed
//////////////////////////////////////////////
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Core {
    // The flag is now an AtomicBool to ensure atomic operations.
    flag: AtomicBool,
}

impl Core {
    // Constructor – initializes the resource as available.
    pub fn new() -> Self {
        Self { flag: AtomicBool::new(true) }
    }

    // This method uses an atomic compare_exchange to safely check and update the flag,
    // ensuring that only one thread can successfully set the flag from true to false.
    pub fn try_use(&self) -> bool {
        if self.flag.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            // Simulate work with a brief delay; the race window is now irrelevant.
            thread::sleep(Duration::from_micros(10));
            return true;
        }
        false
    }
}

fn main() {
    let core = Arc::new(Core::new());
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let core = core.clone();
            thread::spawn(move || {
                if core.try_use() {
                    println!("Resource used");
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}