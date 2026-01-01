/* 
This example demonstrates a flaw where unchecked subtraction leads to an integer underflow.
An unsafe block is used to perform raw pointer arithmetic without validating the arithmetic,
which can wrap the value when subtracting a larger amount than available.
It also employs concurrency and smart pointers to mimic real‐world multithreaded usage.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    value: u32,
}

impl Data {
    fn new(val: u32) -> Self {
        Data { value: val }
    }
    
    // This method performs subtraction unsafely, allowing integer underflow.
    fn subtract(&mut self, amt: u32) -> Result<u32, &'static str> {
        unsafe {
            // Vulnerability: No check for underflow, leading to potential wrap-around.
            let ptr: *mut u32 = &mut self.value as *mut u32;
            // The raw pointer arithmetic here is the root cause of CWE-191.
            *ptr = *ptr - amt;
        }
        Ok(self.value)
    }
}

fn main() {
    // Initialize with 0 so that subtracting 1 will underflow.
    let data = Arc::new(Mutex::new(Data::new(0)));
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut d = data_clone.lock().unwrap();
        d.subtract(1)
    });
    let result = handle.join().unwrap();
    match result {
        Ok(val) => println!("Final value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}