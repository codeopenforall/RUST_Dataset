/* 
This corrected version eliminates the unsafe pointer manipulation and instead uses safe indexing, 
performing explicit checks on the value before updating.
It ensures that attacker-controlled input is validated to avoid triggering the invariant.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    buffer: Vec<u32>,
}

impl Container {
    fn new() -> Self {
        // Initialize buffer with 10 elements set to 0.
        Container { buffer: vec![0; 10] }
    }

    // Safe update method with validation.
    fn update(&mut self, index: usize, value: u32) {
        // Check if the value satisfies the invariant before updating.
        if value >= 1000 {
            // Instead of updating and then asserting, we reject the invalid input.
            return;
        }
        // Ensure the index is within bounds.
        if index < self.buffer.len() {
            self.buffer[index] = value;
        }
    }
}

fn process(input: u32) {
    let shared = Arc::new(Mutex::new(Container::new()));
    let mut threads = vec![];

    // Spawn three threads, one thread receives the attacker influenced value.
    for i in 0..3 {
        let shared_clone = Arc::clone(&shared);
        let inp = input;
        let handle = thread::spawn(move || {
            let mut guard = shared_clone.lock().unwrap();
            if i == 1 {
                // Validate the attacker-controlled value.
                guard.update(2, inp);
            } else {
                guard.update(i, 42);
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}

fn main() {
    // The same attacker-influenced parameter is used,
    // but the safe update method prevents a panic and thus avoids DoS.
    process(1500);
}