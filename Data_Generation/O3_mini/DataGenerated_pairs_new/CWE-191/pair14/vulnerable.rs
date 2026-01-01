//////////////////////////////////////////////////////////////
// Vulnerability Example: Integer Underflow via Unsafe Subtraction
// CWE-191: Integer Underflow
//////////////////////////////////////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

pub struct Data {
    counter: UnsafeCell<u32>,
}

unsafe impl Sync for Data {}

impl Data {
    pub fn new(val: u32) -> Data {
        Data {
            counter: UnsafeCell::new(val),
        }
    }

    // In this method an unchecked subtraction is performed.
    // If sub is greater than the current counter, underflow occurs (wraparound).
    pub fn decrease(&self, sub: u32) -> u32 {
        unsafe {
            // Read the current value
            let current = *self.counter.get();
            // Vulnerability: subtract without checking for underflow.
            let new_val = current.wrapping_sub(sub);
            *self.counter.get() = new_val;
            new_val
        }
    }
}

// This function wraps the operation so that the caller always gets a Result.
// In the vulnerable version the potential underflow is not detected.
pub fn process_operation(initial: u32, sub: u32) -> Result<u32, &'static str> {
    let data = Data::new(initial);
    // Spawn a thread to mimic concurrent usage.
    let shared_data = Arc::new(data);
    let thread_data = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        // No underflow detection here.
        thread_data.decrease(sub)
    });
    let result = handle.join().unwrap();
    Ok(result)
}

fn main() {
    // Example: initial value is 100 and we subtract 200.
    // This causes an underflow leading to a wrapped-around value.
    let res = process_operation(100, 200);
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}