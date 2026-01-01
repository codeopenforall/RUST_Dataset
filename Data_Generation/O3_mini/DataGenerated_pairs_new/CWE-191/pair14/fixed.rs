//////////////////////////////////////////////////////////////
// Fixed Example: Prevent Integer Underflow via Checked Subtraction
// Fix for CWE-191: Integer Underflow
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

    // This version performs a check to prevent underflow.
    pub fn decrease(&self, sub: u32) -> Result<u32, &'static str> {
        unsafe {
            let current = *self.counter.get();
            if current < sub {
                return Err("integer underflow");
            }
            let new_val = current - sub;
            *self.counter.get() = new_val;
            Ok(new_val)
        }
    }
}

// Wraps the fixed operation in a thread to mimic concurrency.
pub fn process_operation(initial: u32, sub: u32) -> Result<u32, &'static str> {
    let data = Data::new(initial);
    let shared_data = Arc::new(data);
    let thread_data = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        // This call now returns a Result with an underflow check.
        thread_data.decrease(sub)
    });
    // Propagate the result from the thread.
    handle.join().unwrap()
}

fn main() {
    // For the fixed version, subtracting 200 from 100 should trigger an error.
    let res = process_operation(100, 200);
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}