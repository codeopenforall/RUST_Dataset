//////////////////////////////////////////////
// Vulnerable Version – Race Condition Issue
//////////////////////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Core {
    // The flag indicates if the resource is available.
    // UnsafeCell is used to allow mutation in an immutable context without proper synchronization.
    flag: UnsafeCell<bool>,
}

impl Core {
    // Constructor – sets the resource as available.
    pub fn new() -> Self {
        Self { flag: UnsafeCell::new(true) }
    }

    // This method checks the flag and then sets it to false
    // to simulate one-time use of the resource.
    // The unsynchronized check-then-use leads to race condition.
    pub fn try_use(&self) -> bool {
        unsafe {
            // Vulnerability: the read and write to the flag are not performed atomically.
            if *self.flag.get() {
                // Artificial delay to widen the race window.
                thread::sleep(Duration::from_micros(10));
                *self.flag.get() = false;
                return true;
            }
        }
        false
    }
}

// Marking Core as Sync unsafely because UnsafeCell<bool> is not thread-safe by default.
unsafe impl Sync for Core {}

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