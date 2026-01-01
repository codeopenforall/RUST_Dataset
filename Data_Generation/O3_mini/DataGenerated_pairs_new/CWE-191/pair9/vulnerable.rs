////////////////////////////////////////////////////////////////////////////////
// THIS IS THE VULNERABLE IMPLEMENTATION (for demonstration)
// CWE-191: Integer Underflow due to unchecked subtraction in unsafe block
////////////////////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new(n: u32) -> Self {
        Self { count: n }
    }

    // This method subtracts without checking for underflow.
    // An unsafe block is used to perform unchecked pointer arithmetic.
    pub fn subtract(&mut self, val: u32) {
        unsafe {
            let ptr = &mut self.count as *mut u32;
            // Vulnerability: when self.count is 0, subtraction underflows and wraps.
            *ptr = *ptr - val;
        }
    }

    pub fn get_value(&self) -> u32 {
        self.count
    }
}

// This function encapsulates the calculation logic.
// It spawns a thread that calls subtract(), inducing an underflow when starting from 0.
pub fn execute_calculation() -> u32 {
    let counter = Arc::new(Mutex::new(Counter::new(0)));
    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut lock = counter_clone.lock().unwrap();
        lock.subtract(1);
    });
    handle.join().unwrap();
    let final_value = counter.lock().unwrap().get_value();
    final_value
}

fn main() {
    let res = execute_calculation();
    println!("Final value: {}", res);
}