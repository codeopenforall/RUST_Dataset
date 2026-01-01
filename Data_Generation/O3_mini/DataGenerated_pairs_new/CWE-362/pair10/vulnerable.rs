///////////////////////////////////////////////////////////////////////////////
// This code implements a shared counter using an unsynchronized UnsafeCell.
// Multiple threads concurrently check and update the counter without locks.
// An artificial delay (sleep) is introduced between the check and update,
// creating a TOCTOU race condition. In a correctly synchronized system, the
// final counter should equal 10000, but due to the race condition, many increments
// are lost and the result is nondeterministic (and typically less than 10000).
///////////////////////////////////////////////////////////////////////////////

use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Shared {
    // UnsafeCell permits interior mutability without any thread-safety guarantees.
    counter: UnsafeCell<u32>,
}

// SAFETY: We explicitly state that Shared is Sync, even though it uses UnsafeCell,
// because we want to simulate a real-world vulnerability stemming from unsynchronized access.
unsafe impl Sync for Shared {}

impl Shared {
    fn new() -> Self {
        Shared {
            counter: UnsafeCell::new(0),
        }
    }
}

// The application routine that spawns several threads to increment the counter.
fn run_app() -> u32 {
    let shared = Arc::new(Shared::new());
    let mut handles = Vec::new();
    let iterations_per_thread = 1000;
    // Spawn 10 threads; expected final result if properly synchronized would be 10 * 1000 = 10000.
    for _ in 0..10 {
        let data = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..iterations_per_thread {
                // Introduce a window between check and update to exacerbate the race.
                unsafe {
                    let current = *data.counter.get();
                    if current < 10000 {
                        // Sleep briefly to mimic context switch and allow interleaving.
                        thread::sleep(Duration::from_micros(1));
                        *data.counter.get() = current + 1;
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete.
    for h in handles {
        h.join().expect("Thread panicked");
    }

    // Return the final counter value.
    unsafe { *shared.counter.get() }
}

fn main() {
    let result = run_app();
    println!("Final counter: {}", result);
}