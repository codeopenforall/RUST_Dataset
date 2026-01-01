////////////////////////////////////////////////////
// Corrected Code: Synchronized concurrent update   //
// CWE-362: Race Condition Fixed using Mutex locking  //
////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Data {
    count: usize,
}

/// Executes the main concurrent processing and returns the final count.
/// Uses a Mutex wrapped in an Arc to guarantee proper synchronization.
pub fn run_process() -> usize {
    let shared = Arc::new(Mutex::new(Data { count: 0 }));
    let mut handles = Vec::new();
    // Spawn 10 threads; each will obtain the lock before updating.
    for _ in 0..10 {
        let shared_clone = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            let mut data = shared_clone.lock().unwrap();
            // Check-then-act sequence is now atomic because of the Mutex.
            if data.count % 2 == 0 {
                thread::sleep(Duration::from_millis(10));
                data.count += 1;
            } else {
                thread::sleep(Duration::from_millis(10));
                data.count += 2;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_data = shared.lock().unwrap();
    final_data.count
}

fn main() {
    let result = run_process();
    println!("Final count (synchronized): {}", result);
}