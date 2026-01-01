use std::sync::{Arc, Mutex};
use std::thread;

struct Manager {
    // Internal data buffer.
    buffer: Vec<u32>,
    // A marker used to simulate additional internal state.
    marker: u32,
}

impl Manager {
    fn new(size: usize) -> Self {
        Manager {
            buffer: vec![0; size],
            marker: 0,
        }
    }

    // This function takes an index and unsafely writes to the internal buffer.
    // The index comes from an external (attacker-influenced) source.
    fn process(&mut self, idx: usize) -> Result<(), &'static str> {
        let len = self.buffer.len();
        // Unsafe block: directly obtaining a mutable pointer to the buffer.
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            // The following assertions use the attacker-supplied index.
            // CWE-617: A reachable assertion triggered by an attacker-controlled value.
            debug_assert!(idx < len, "Index out-of-bounds in debug mode");
            assert!(idx < len, "Index out-of-bounds by attacker input");
            // Write to memory without a safe bounds-check.
            *ptr.add(idx) = 42;
        }
        self.marker = 1;
        Ok(())
    }
}

fn main() {
    // Wrap the Manager in Arc and Mutex to simulate concurrent access.
    let manager = Arc::new(Mutex::new(Manager::new(10)));

    // Spawn a concurrent thread that uses an attacker-controlled index.
    let mgr_clone = Arc::clone(&manager);
    let t = thread::spawn(move || {
        let mut m = mgr_clone.lock().unwrap();
        // Attacker-controlled input: intentionally out-of-bound index.
        // This triggers the assertion in the unsafe block, causing a panic.
        let _ = m.process(15);
    });

    // If the assertion is reached, this join will panic.
    let _ = t.join().unwrap();
    println!("Finished processing in vulnerable version");
}