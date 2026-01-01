use std::sync::{Arc, Mutex};
use std::thread;

struct Computor {
    value: u64,
}

impl Computor {
    fn new(val: u64) -> Self {
        Computor { value: val }
    }
    
    // Unsafe subtraction without underflow check.
    unsafe fn subtract(&mut self, sub: u64) {
        // Simulate vulnerability by using raw pointer arithmetic.
        let ptr = &mut self.value as *mut u64;
        *ptr = *ptr - sub; // Vulnerable subtraction: underflow wraparound may occur.
    }
    
    fn get(&self) -> u64 {
        self.value
    }
}

// A public API function that processes the operation.
// When given triggering input, it will perform unchecked subtraction.
pub fn process(input: u64, sub: u64) -> Result<u64, &'static str> {
    let mut comp = Computor::new(input);
    unsafe {
        comp.subtract(sub);
    }
    Ok(comp.get())
}

fn main() {
    // Demonstration with safe parameters.
    match process(100, 10) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}