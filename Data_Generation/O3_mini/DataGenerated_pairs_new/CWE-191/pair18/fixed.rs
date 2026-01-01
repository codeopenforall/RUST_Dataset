//////////////////////////////
// Corrected Code Example   //
// CWE-191: Integer Underflow Mitigation
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

    // This method uses checked subtraction to prevent underflow.
    // It panics if the subtraction would cause an underflow.
    fn decrement(&self, amount: u32) {
        let mut guard = self.data.lock().unwrap();
        let current = *guard;
        // Fixed: Use checked subtraction to safely handle the operation.
        match current.checked_sub(amount) {
            Some(new_val) => *guard = new_val,
            None => panic!("Underflow detected: cannot subtract {} from {}", amount, current),
        }
    }

    fn fetch(&self) -> u32 {
        *self.data.lock().unwrap()
    }
}

fn run() -> u32 {
    let calc = Arc::new(Calculator::new(5));
    let calc_clone = Arc::clone(&calc);

    let handler = thread::spawn(move || {
        // This will panic due to underflow prevention.
        calc_clone.decrement(10);
    });
    // In a normal run the panic will propagate from the thread.
    handler.join().unwrap();

    calc.fetch()
}

fn main() {
    // The main function triggers the operation.
    // Underflow will be caught by the checks in decrement().
    let _ = run();
}