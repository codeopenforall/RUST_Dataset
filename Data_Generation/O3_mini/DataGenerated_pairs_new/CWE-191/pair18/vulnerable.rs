//////////////////////////////
// Vulnerable Code Example  //
// CWE-191: Integer Underflow
//////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Calculator {
    data: Mutex<u32>,
}

impl Calculator {
    fn new(initial: u32) -> Self {
        Calculator {
            data: Mutex::new(initial),
        }
    }

    // This method subtracts without checking for underflow.
    // It uses an unsafe block and raw pointer manipulation to simulate
    // a real-world vulnerability in a concurrent context.
    fn decrement(&self, amount: u32) {
        unsafe {
            let mut guard = self.data.lock().unwrap();
            let current = *guard;
            // Simulate unsafe pointer usage that bypasses typical checks.
            let ptr: *mut u32 = &mut *guard;
            // Vulnerability: subtracting without checking leads to potential underflow.
            *ptr = current - amount;
        }
    }

    fn fetch(&self) -> u32 {
        *self.data.lock().unwrap()
    }
}

fn run() -> u32 {
    // Initialize with a small unsigned integer.
    let calc = Arc::new(Calculator::new(5));
    let calc_clone = Arc::clone(&calc);

    // Spawn a thread to perform the subtraction concurrently.
    let handler = thread::spawn(move || {
        // This subtraction will cause an underflow because 5 - 10 wraps around.
        calc_clone.decrement(10);
    });
    handler.join().unwrap();

    calc.fetch()
}

fn main() {
    let result = run();
    println!("Result: {}", result);
}