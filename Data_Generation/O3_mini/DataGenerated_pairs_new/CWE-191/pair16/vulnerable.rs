///////////////////////////////////////////////////////////////
// Vulnerable Rust code demonstrating integer underflow (CWE-191)
// This program uses unsafe pointer arithmetic and concurrent threads
// to subtract an unsigned integer without boundary checks.
// The function always returns Ok(result) even if underflow occurs.
///////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    value: u32,
}

impl Container {
    // Subtracts 'amount' from value without any underflow protection.
    // Unsafe block used to mimic low-level operations.
    fn subtract_unchecked(&mut self, amount: u32) -> u32 {
        unsafe {
            // Vulnerability occurs in this unsafe pointer arithmetic.
            let ptr: *mut u32 = &mut self.value;           // [lineno: 35]
            *ptr = *ptr - amount;                           // [lineno: 36]
            *ptr
        }
    }
}

fn process(data: Arc<Mutex<Container>>, amount: u32) -> Result<u32, &'static str> {
    // Spawn a thread to simulate concurrent access.
    let handle = thread::spawn(move || {
        let mut container = data.lock().unwrap();
        // Call the unchecked subtraction.
        let res = container.subtract_unchecked(amount);
        // Wrap the result in Ok despite possible integer underflow.
        Ok(res)
    });
    handle.join().unwrap()
}

fn main() {
    // Initialize Container with 0 so that subtracting 1 causes underflow.
    let container = Container { value: 0 };
    let shared = Arc::new(Mutex::new(container));
    // Underflow: subtract 1 from 0.
    let result = process(shared.clone(), 1);
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}