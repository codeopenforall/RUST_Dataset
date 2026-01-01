//////////////////////////// Fixed Version ////////////////////////////
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

    // Secure method that validates the offset and count before performing the operation.
    fn validated_sum(&self, offset: usize, count: usize) -> Result<i32, &'static str> {
        if offset.checked_add(count).filter(|&sum| sum <= self.buffer.len()).is_none() {
            return Err("Invalid offset or count");
        }
        // Safe to create the slice since the precondition is verified.
        unsafe {
            let ptr = self.buffer.as_ptr().add(offset);
            let slice = std::slice::from_raw_parts(ptr, count);
            Ok(slice.iter().sum())
        }
    }

    // This safe wrapper calls the validated_sum() and returns its result.
    fn safe_entry(&self, offset: usize, count: usize) -> Result<i32, &'static str> {
        self.validated_sum(offset, count)
    }
}

fn main() {
    let block = MemoryBlock::new();
    // A valid call: using [1, 3] gives 20+30+40 = 90.
    let valid = block.safe_entry(1, 3);
    println!("Valid call result: {:?}", valid);

    // An invalid call: offset 2 with count 4 is detected and returns an error.
    let invalid = block.safe_entry(2, 4);
    println!("Invalid call result: {:?}", invalid);
}