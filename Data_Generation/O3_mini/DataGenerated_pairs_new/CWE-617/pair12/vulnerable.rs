////////////////////////////////////////////////////////////////////////////////////////////////////
// Vulnerable Version: This program implements a worker that updates an internal counter
// using an unsafe pointer write. The update method does not validate attacker‐supplied input 
// before performing an unsafe assignment and then relies on an assertion to enforce that the 
// counter does not exceed the preset limit. An attacker can supply a value (e.g. 150 when the 
// limit is 100) to trigger the reachable assertion, causing a panic and a potential denial‐of‐service.
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Worker {
    counter: usize,
    limit: usize,
}

impl Worker {
    fn new(limit: usize) -> Self {
        Worker { counter: 0, limit }
    }

    // Performs an unsafe update without prior validation.
    fn update(&mut self, new_value: usize) {
        unsafe {
            let ptr = &mut self.counter as *mut usize;
            // Write the new value directly into counter.
            *ptr = new_value;
        }
        // Reachable assertion: if the new value is above the limit (attacker controlled),
        // this assertion will fail and panic.
        assert!(self.counter <= self.limit, "Value exceeds maximum allowed");
    }
}

// Public API intended for unit testing.
// For valid input (<= limit) it returns Ok(()), but if the input is larger the assertion triggers panic.
pub fn process_input(input: usize) -> Result<(), String> {
    let mut worker = Worker::new(100);
    // This call will panic if input > 100.
    worker.update(input);
    Ok(())
}

fn main() {
    // Spawn a thread to simulate concurrent processing of attacker-controlled input.
    let worker = Arc::new(Mutex::new(Worker::new(100)));
    let worker_clone = Arc::clone(&worker);

    let handle = thread::spawn(move || {
        let mut worker = worker_clone.lock().unwrap();
        // Attacker-controlled value; for example, 150 exceeds the limit and triggers panic.
        worker.update(150);
    });

    let _ = handle.join();
    println!("Completed processing in vulnerable version.");
}