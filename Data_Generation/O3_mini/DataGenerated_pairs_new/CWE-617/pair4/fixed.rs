////////////////////////////
// Fixed Code Sample
////////////////////////////

// In the fixed version, we remove the race condition by protecting the state update with a Mutex,
// ensuring that the invariant remains consistent throughout the operation. The background thread now
// refrains from lowering the bound below any valid index, and the main thread reliably verifies the invariant.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Processor {
    state: Arc<Mutex<usize>>,
    buffer: Vec<u8>,
}

impl Processor {
    fn new() -> Self {
        // Initialize with a bound of 64 and a buffer of 64 zeros.
        Self {
            state: Arc::new(Mutex::new(64)),
            buffer: vec![0; 64],
        }
    }

    fn execute(&self, index: usize) {
        // Spawn a background thread that attempts to update the bound.
        // With proper synchronization, the bound is guarded to avoid unsafe modifications.
        let state_handle = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            let mut bound = state_handle.lock().unwrap();
            // Instead of lowering the bound arbitrarily, ensure that it does not drop below the requested index.
            if *bound > index {
                // In this fixed code, we ensure that the invariant is preserved.
                // (Alternatively, this thread might perform other safe maintenance operations.)
                *bound = *bound;
            }
        });
        // Wait to allow the background thread to complete its controlled update.
        thread::sleep(Duration::from_millis(20));

        // Lock the state to get a consistent snapshot of the bound.
        let current_bound = *self.state.lock().unwrap();

        unsafe {
            let ptr = self.buffer.as_ptr().add(index);
            // Now the invariant check uses a reliably obtained current_bound.
            assert!(index < current_bound, "Invariant violation: index out of bound");
            println!("Buffer value: {}", *ptr);
        }

        // Ensure the background thread has completed.
        handle.join().expect("Thread join failed");
    }
}

fn main() {
    let proc = Processor::new();
    // A valid index (32) should reliably pass the invariant check in the fixed version.
    proc.execute(32);
}