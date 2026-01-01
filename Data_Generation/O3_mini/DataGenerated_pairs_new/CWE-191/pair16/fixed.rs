///////////////////////////////////////////////////////////////
// Corrected Rust code preventing integer underflow (CWE-191)
// This program uses checked arithmetic to ensure that subtraction
// does not result in underflow. It mimics realistic concurrency
// patterns and uses a similar API as the vulnerable version.
///////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    value: u32,
}

impl Container {
    // Subtracts 'amount' from value after checking for underflow.
    fn subtract_checked(&mut self, amount: u32) -> Result<u32, &'static str> {
        if self.value < amount {
            // Detected underflow.
            return Err("Integer underflow detected");
        }
        self.value = self.value - amount;
        Ok(self.value)
    }
}

fn process(data: Arc<Mutex<Container>>, amount: u32) -> Result<u32, &'static str> {
    // Simulate concurrent processing.
    let handle = thread::spawn(move || {
        let mut container = data.lock().unwrap();
        container.subtract_checked(amount)
    });
    handle.join().unwrap()
}

fn main() {
    // Initialize Container with 0; any subtraction that is too large will be caught.
    let container = Container { value: 0 };
    let shared = Arc::new(Mutex::new(container));
    // Attempt to subtract 1 from 0; should trigger underflow protection.
    match process(shared.clone(), 1) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}