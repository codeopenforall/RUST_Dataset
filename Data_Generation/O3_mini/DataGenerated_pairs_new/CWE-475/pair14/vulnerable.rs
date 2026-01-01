//////////////////////////// Vulnerable Version ////////////////////////////
use std::panic;

struct MemoryBlock {
    buffer: Box<[i32]>,
}

impl MemoryBlock {
    fn new() -> Self {
        // Fixed data buffer of length 5.
        Self {
            buffer: vec![10, 20, 30, 40, 50].into_boxed_slice(),
        }
    }

    // UNSAFE internal method that computes the sum from an offset with a given count
    // without validating that the region [offset, offset+count) is within bounds.
    fn dangerous_sum(&self, offset: usize, count: usize) -> i32 {
        // This unsafe block violates the API precondition:
        // calling from_raw_parts with an invalid length (offset+count exceeds the allocated slice)
        unsafe {
            let ptr = self.buffer.as_ptr().add(offset);
            // Vulnerable: no check is performed on count.
            let slice = std::slice::from_raw_parts(ptr, count);
            slice.iter().sum()
        }
    }

    // This safe wrapper catches panics to convert the operation into a Result.
    // In this vulnerable version, no proper validation is done.
    fn safe_entry(&self, offset: usize, count: usize) -> Result<i32, &'static str> {
        let res = panic::catch_unwind(|| self.dangerous_sum(offset, count));
        match res {
            Ok(val) => Ok(val),
            Err(_) => Err("Operation panicked"),
        }
    }
}

fn main() {
    let block = MemoryBlock::new();
    // For demonstration:
    // A valid call: using [1, 3] gives 20+30+40 = 90.
    let valid = block.safe_entry(1, 3);
    println!("Valid call result: {:?}", valid);

    // An invalid call: offset 2 with count 4 goes beyond the allocated array (2+4 > 5).
    // This violates the API precondition, leading to undefined behavior.
    let invalid = block.safe_entry(2, 4);
    println!("Invalid call result: {:?}", invalid);
}