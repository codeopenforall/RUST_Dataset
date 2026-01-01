//////////////////////////////
// Fixed Code Example
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

    // Safely subtract 'amount' from total delaying underflow:
    // if the subtraction would underflow, it sets the total to zero.
    fn update(&mut self, amount: u32) {
        // Using checked_sub returns Option<u32>.
        self.total = self.total.checked_sub(amount).unwrap_or(0);
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
    // With the fix, subtracting from zero safely results in zero.
    println!("Result: {}", acc.value());
}