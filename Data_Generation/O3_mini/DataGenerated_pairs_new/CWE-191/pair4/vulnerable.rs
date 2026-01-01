use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    value: u64,
}

impl Counter {
    fn new(init: u64) -> Self {
        Counter { value: init }
    }
    
    // This method subtracts the provided amount without checking for underflow.
    // An unsafe block is used here to mimic low-level pointer manipulation.
    fn decrease(&mut self, amount: u64) {
        unsafe {
            // Vulnerability: using unchecked subtraction that will wrap on underflow.
            // In a scenario where amount exceeds self.value, this causes an integer underflow.
            self.value = self.value.wrapping_sub(amount);
        }
    }
    
    fn get(&self) -> u64 {
        self.value
    }
}

// Spawns multiple threads to concurrently reduce the counter.
fn run(counter: Arc<Mutex<Counter>>, amount: u64, iterations: u32) {
    let mut handles = vec![];
    for _ in 0..iterations {
        let c = counter.clone();
        handles.push(thread::spawn(move || {
            let mut lock = c.lock().unwrap();
            lock.decrease(amount);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    // Starting the counter at 50.
    let counter = Arc::new(Mutex::new(Counter::new(50)));
    // Subtract 10, ten times. Ideally, the result should not become negative,
    // but the unchecked arithmetic may cause an underflow, resulting in a wraparound.
    run(counter.clone(), 10, 10);
    let final_value = counter.lock().unwrap().get();
    println!("Final count: {}", final_value);
}