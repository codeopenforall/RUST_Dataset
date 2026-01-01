///////////////////////////////////////////////////////////////////////////////
// This corrected version eliminates the race condition by protecting the shared counter
// with a Mutex. All accesses to the counter become atomic with respect to each other.
// The artificial delay is retained to mimic realistic execution delays, but the Mutex ensures
// sequential consistency, so the final counter is deterministically 10000.
///////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Shared {
    counter: Mutex<u32>,
}

impl Shared {
    fn new() -> Self {
        Shared {
            counter: Mutex::new(0),
        }
    }
}

fn run_app() -> u32 {
    let shared = Arc::new(Shared::new());
    let mut handles = Vec::new();
    let iterations_per_thread = 1000;
    for _ in 0..10 {
        let data = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..iterations_per_thread {
                // Lock the mutex to obtain exclusive access.
                let mut guard = data.counter.lock().unwrap();
                if *guard < 10000 {
                    // Retain the artificial delay to mimic realistic conditions.
                    thread::sleep(Duration::from_micros(1));
                    *guard += 1;
                }
                // Mutex is unlocked automatically when guard goes out of scope.
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().expect("Thread panicked");
    }

    let final_val = *shared.counter.lock().unwrap();
    final_val
}

fn main() {
    let result = run_app();
    println!("Final counter: {}", result);
}