use std::sync::{Arc, Mutex};
use std::thread;

struct Computor {
    value: u64,
}

impl Computor {
    fn new(val: u64) -> Self {
        Computor { value: val }
    }
    
    // Safe subtraction with an explicit underflow check.
    fn subtract(&mut self, sub: u64) -> Result<(), &'static str> {
        if self.value < sub {
            return Err("Underflow detected");
        }
        self.value -= sub;
        Ok(())
    }
    
    fn get(&self) -> u64 {
        self.value
    }
}

// The public API now uses safe arithmetic. If the operation would cause an underflow,
// an error is returned instead of performing a wraparound.
pub fn process(input: u64, sub: u64) -> Result<u64, &'static str> {
    let mut comp = Computor::new(input);
    comp.subtract(sub)?;
    Ok(comp.get())
}

fn main() {
    // Demonstration with safe parameters.
    match process(100, 10) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}