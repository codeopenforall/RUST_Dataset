///////////////////////////////////////////////////////////////
// Corrected Code: Bounds-check added to prevent memory overflow (CWE-119 fix)
///////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};

struct MemoryManager {
    data: Box<[u8; 10]>,
}

impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager { data: Box::new([0; 10]) }
    }

    // The update method now checks that the index is within bounds.
    // If the index is invalid, an error is returned instead of performing the write.
    pub fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn sum(&self) -> u32 {
        self.data.iter().map(|&x| x as u32).sum()
    }
}

// The processing function now returns an error for out-of-bound updates.
fn process() -> Result<u32, &'static str> {
    let mut mgr = MemoryManager::new();
    // This update call will detect that index 10 is out of bounds,
    // and update() will return an Err instead of writing.
    mgr.update(10, 42)?;
    Ok(mgr.sum())
}

fn main() {
    // The main function now properly handles the error case.
    match process() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}