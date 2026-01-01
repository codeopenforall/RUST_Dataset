use std::sync::{Arc, Mutex};
use std::thread;

struct ProcessData {
    value: i32,
}

impl ProcessData {
    fn new(val: i32) -> Self {
        ProcessData { value: val }
    }

    // Secure update that only applies a new value if it is within acceptable bounds.
    fn alter(&mut self, new_val: i32) {
        // Validate the update to maintain the invariant.
        if new_val < 100 {
            self.value = new_val;
        }
        // If new_val does not meet the criteria, the update is rejected.
    }
}

fn main() {
    // Shared state is encapsulated in an Arc and Mutex.
    let data = Arc::new(Mutex::new(ProcessData::new(10)));
    let shared = Arc::clone(&data);

    // Spawn a thread that attempts to update the state.
    let handle = thread::spawn(move || {
        if let Ok(mut state) = shared.lock() {
            state.alter(999); // Malicious update is ignored, preserving the invariant.
        }
    });

    handle.join().unwrap();

    // The invariant remains intact: value is still below 100.
    let state = data.lock().unwrap();
    assert!(state.value < 100, "Invariant violation: value is too high!");
    println!("Completed with value: {}", state.value);
}