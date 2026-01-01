//////////////////////////////////////////////////////////////
// Fixed Example: Safe Concurrent Handling of Attacker Input
// This revised version replaces the unsafe update with a safe update routine.
// The input value is validated before updating the shared state. If the value
// exceeds the safe maximum (10), it is clamped to 10. This prevents the reachable
// assertion from being triggered.
//////////////////////////////////////////////////////////////

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    threshold: u32,
}

impl Data {
    fn new(val: u32) -> Self {
        Data { threshold: val }
    }

    // Safe update function that validates the new value.
    fn secure_update(&mut self, val: u32) {
        // Clamp the value to 10 if it would exceed the safe limit.
        self.threshold = if val <= 10 { val } else { 10 };
    }
}

fn gather_input(input: u32) -> Arc<Mutex<Data>> {
    let shared = Arc::new(Mutex::new(Data::new(5)));
    {
        let shared_clone = Arc::clone(&shared);
        // Spawn a thread to simulate asynchronous update.
        thread::spawn(move || {
            let mut data_guard = shared_clone.lock().unwrap();
            data_guard.secure_update(input);
        })
        .join()
        .unwrap();
    }
    shared
}

fn main() {
    // Read input from command-line arguments, representing attacker-influenced data.
    let args: Vec<String> = env::args().collect();
    let input: u32 = if args.len() > 1 {
        args[1].parse().unwrap_or(5)
    } else {
        5
    };
    let shared = gather_input(input);
    let data = shared.lock().unwrap();
    // The assertion now serves as a sanity check since secure_update ensures
    // the threshold never exceeds 10.
    assert!(data.threshold <= 10, "Threshold exceeded safe limit");
    println!("Threshold is safe: {}", data.threshold);
}