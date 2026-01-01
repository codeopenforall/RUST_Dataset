use std::sync::{Arc, Mutex};
use std::thread;

struct ProcessData {
    value: i32,
}

impl ProcessData {
    fn new(val: i32) -> Self {
        ProcessData { value: val }
    }

    // Insecure method that unsafely sets the state without validating the input.
    fn alter(&mut self, new_val: i32) {
        // Unsafe raw pointer manipulation bypasses any logical checks.
        unsafe {
            let ptr: *mut i32 = &mut self.value;
            *ptr = new_val; // Directly set to new_val, even if it violates the invariant.
        }
    }
}

fn main() {
    // Shared state protected by a Mutex wrapped in an Arc.
    let data = Arc::new(Mutex::new(ProcessData::new(10)));
    let shared = Arc::clone(&data);

    // Spawn a thread that maliciously tries to update the state.
    let handle = thread::spawn(move || {
        if let Ok(mut state) = shared.lock() {
            state.alter(999); // Unsafe update that violates the invariant.
        }
    });

    handle.join().unwrap();

    // Invariant: the state must remain less than 100.
    let state = data.lock().unwrap();
    assert!(state.value < 100, "Invariant violation: value is too high!");
    println!("Completed with value: {}", state.value);
}