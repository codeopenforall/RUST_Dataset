////////////////////////////////////////////////////////////////////////////////////////////////////
// Fixed Version: This implementation adds proper validation prior to updating the internal counter.
// The update method now safely checks whether the input is within the allowed limit and returns a
// Result instead of panicking. In concurrent execution, attacker-controlled input is gracefully 
// rejected rather than causing a disruption.
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

    // Safely update the counter after checking if the new value is within allowed limits.
    fn update(&mut self, new_value: usize) -> Result<(), String> {
        if new_value > self.limit {
            return Err("Attempt to set value above allowed maximum".to_owned());
        }
        self.counter = new_value;
        Ok(())
    }
}

// Public API for unit testing.
// This function will return Err when supplied an input exceeding the limit,
// but will not panic.
pub fn process_input(input: usize) -> Result<(), String> {
    let mut worker = Worker::new(100);
    worker.update(input)
}

fn main() {
    // Spawn a thread to handle the input concurrently.
    let worker = Arc::new(Mutex::new(Worker::new(100)));
    let worker_clone = Arc::clone(&worker);

    let handle = thread::spawn(move || {
        let mut worker = worker_clone.lock().unwrap();
        // Attacker-controlled input; safe update returns an error rather than panicking.
        let res = worker.update(150);
        assert!(res.is_err(), "Expected error for input exceeding limit.");
    });

    let _ = handle.join();
    println!("Completed processing in fixed version.");
}