//////////////////////////////
// Vulnerable Code Example
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Accumulator {
    total: u32,
}

impl Accumulator {
    // Create a new accumulator with an initial value.
    fn new(initial: u32) -> Self {
        Accumulator { total: initial }
    }

    // This method subtracts 'amount' from the total without preventing underflow.
    // It uses an unsafe block with raw pointer dereferencing.
    fn update(&mut self, amount: u32) {
        unsafe {
            let ptr: *mut u32 = &mut self.total;
            // Vulnerability: subtracting without checking for underflow.
            *ptr = (*ptr) - amount;
        }
    }

    fn value(&self) -> u32 {
        self.total
    }
}

fn main() {
    // Shared accumulator wrapped in Arc and Mutex to simulate concurrent updates.
    let data = Arc::new(Mutex::new(Accumulator::new(0)));
    let mut handles = vec![];

    // Spawn multiple threads to concurrently subtract 1.
    for _ in 0..4 {
        let data_cloned = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut acc = data_cloned.lock().unwrap();
            acc.update(1);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let acc = data.lock().unwrap();
    // In case of underflow, the result wraps around to a very high value.
    println!("Result: {}", acc.value());
}