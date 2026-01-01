///////////////////////////////////////////////////////////////
// Vulnerable Code: Improper Restriction of Operations within Memory Bounds (CWE-119)
///////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};

struct MemoryHandler {
    data: Box<[u8; 10]>,
}

impl MemoryHandler {
    pub fn new() -> Self {
        MemoryHandler { data: Box::new([0; 10]) }
    }

    // This method uses an unsafe block without any bounds checking.
    // It writes to a memory location computed by pointer arithmetic.
    pub fn update(&mut self, index: usize, value: u8) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            // POTENTIAL FLAW:
            // No bounds check is done on index.
            // If index >= 10, this leads to an out-of-bound write.
            *ptr.add(index) = value;
        }
    }

    pub fn sum(&self) -> u32 {
        self.data.iter().map(|&x| x as u32).sum()
    }
}

// The processing function triggers the vulnerability
// by attempting to write at index 10 (which is out of bounds).
// The return type is unified to Result for testing purposes.
fn process() -> Result<u32, &'static str> {
    let mut handler = MemoryHandler::new();
    // Trigger the unsafe out-of-bound write.
    handler.update(10, 42);
    Ok(handler.sum())
}

fn main() {
    // In normal execution, the program simply prints the computed sum.
    // Due to the unsafe out-of-bound write, undefined behavior may occur.
    match process() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}