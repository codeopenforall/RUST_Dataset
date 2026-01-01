//////////////////////////////////////////////////////////////
// The following code fixes the race condition by replacing the unsynchronized
// counter update with atomic operations. By utilizing AtomicU32 with sequentially
// consistent ordering, we ensure that concurrent increments are performed safely.
//////////////////////////////////////////////////////////////

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;

/// Executes a concurrent increment routine with proper synchronization using atomics.
/// Spawns 10 threads where each thread loops 1000 times to perform an atomic increment.
/// Returns the final counter value.
pub fn run() -> u32 {
    let counter = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    // Wait for all threads to complete.
    for handle in handles {
        let _ = handle.join();
    }

    counter.load(Ordering::SeqCst)
}

fn main() {
    let final_count = run();
    println!("Final counter value: {}", final_count);
}