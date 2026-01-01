////////////////////////////////////////////////////////////////////////////////////////////////////
// A realistic multi-threaded service that processes update requests is implemented below.
// The service uses shared mutable state and unsafe pointer arithmetic. It accepts a multiplier
// value that is influenced by an external (attacker-controlled) input. In the update routine,
// after unsafely updating an internal counter, an assertion is made that the counter remains
// below a fixed threshold. An attacker can supply a multiplier that forces the counter to exceed
// this threshold, triggering the assertion and causing a denial of service.
// This simulates CWE-617 where a reachable assertion on attacker-influenced state leads to DoS.
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

const THRESHOLD: usize = 100;

struct Service {
    value: usize,
}

impl Service {
    fn new() -> Self {
        Service { value: 1 }
    }

    // The update method unsafely modifies the counter with a given multiplier.
    // The assertion below is reachable and depends on attacker-controlled input.
    fn update(&mut self, multiplier: usize) {
        // Unsafe block: directly manipulate memory location of self.value.
        let ptr: *mut usize = &mut self.value;
        unsafe {
            // Potentially dangerous multiplication leading to overflow or threshold breach.
            *ptr = self.value.wrapping_mul(multiplier);
        }
        // Reachable assertion based on attacker-influenced state.
        // CWE-617: an attacker can supply a multiplier that makes self.value >= THRESHOLD,
        // causing the assertion to fail and resulting in a DoS.
        assert!(self.value < THRESHOLD, "Error: value exceeded safe threshold!");
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(Service::new()));
    let mut threads = vec![];

    // Spawn a set of threads that perform updates.
    // One of these threads uses a malicious multiplier.
    for i in 1..=5 {
        let service_clone = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            let mut instance = service_clone.lock().unwrap();
            // Simulate attacker-controlled input: thread 3 uses a high multiplier.
            let factor = if i == 3 { 150 } else { 2 };
            instance.update(factor);
        }));
    }

    for handle in threads {
        // Join threads; a panic in any thread (due to the assertion) will terminate the program.
        let _ = handle.join();
    }
    println!("Final value: {}", shared.lock().unwrap().value);
}