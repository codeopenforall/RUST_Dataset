//////////////////////////
// Vulnerable Code Sample
//////////////////////////

// CWE-617: Reachable Assertion via attacker-controlled concurrent state modification.
//
// This code uses an atomic value shared among threads to enforce an invariant on an index.
// However, a concurrently spawned thread unsafely lowers the bound before the unsafe block’s
// assertion is executed. An attacker can indirectly force the assertion to panic (DoS) by
// triggering the race condition.
use std::sync::{Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

struct Processor {
    state: Arc<AtomicUsize>,
    buffer: Vec<u8>,
}

impl Processor {
    fn new() -> Self {
        // Initialize with a bound of 64 and a buffer of 64 zeros.
        Self {
            state: Arc::new(AtomicUsize::new(64)),
            buffer: vec![0; 64],
        }
    }

    fn execute(&self, index: usize) {
        // Spawn a background thread that simulates an attacker-influenced modification.
        let state_handle = Arc::clone(&self.state);
        thread::spawn(move || {
            // Wait briefly and then lower the valid bound to zero.
            thread::sleep(Duration::from_millis(10));
            state_handle.store(0, Ordering::Relaxed); // Vulnerability trigger at line 19.
        });
        // Wait to allow potential state update by the background thread.
        thread::sleep(Duration::from_millis(20));

        unsafe {
            let ptr = self.buffer.as_ptr().add(index); // Compute pointer with potential index.
            let current_bound = self.state.load(Ordering::Relaxed);
            // Reachable assertion: if index (attacker influenced) is not less than current_bound,
            // the condition fails, triggering a panic (denial of service).
            assert!(index < current_bound, "Invariant violation: index out of bound"); // Vulnerability occurs here (line 25).
            println!("Buffer value: {}", *ptr);
        }
    }
}

fn main() {
    let proc = Processor::new();
    // An attacker-controlled normal index (e.g., 32) might unexpectedly violate the invariant.
    proc.execute(32);
}