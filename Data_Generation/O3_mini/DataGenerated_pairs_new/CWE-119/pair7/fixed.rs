//////////////////// Fixed Code ////////////////////
use std::ptr;

struct Data {
    buffer: [u8; 64],
    magic: u32,
}

impl Data {
    pub fn new() -> Self {
        Self {
            buffer: [0; 64],
            magic: 0xDEADBEEF,
        }
    }

    // The corrected operate() function now checks that the input length does not exceed
    // the buffer capacity, preventing any out-of-bounds memory writes.
    pub fn operate(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.buffer.len() {
            return Err("Input size exceeds buffer capacity");
        }
        unsafe {
            let dest = self.buffer.as_mut_ptr();
            // Safe copy since the length is verified.
            for i in 0..input.len() {
                *dest.add(i) = input[i];
            }
        }
        Ok(())
    }

    // Ensures that the "magic" field remains unmodified.
    pub fn check(&self) -> bool {
        self.magic == 0xDEADBEEF
    }
}

fn main() {
    let mut obj = Data::new();
    // Provide an input that would trigger the boundary violation in the vulnerable version.
    let input = vec![1u8; 100];
    // The operate() method now safely rejects oversized input.
    let res = obj.operate(&input);
    // Ensure that the operation has failed due to input size.
    assert!(res.is_err(), "Operation should fail for oversized input");
    println!("Operation rejected oversized input safely");
}