//////////////////// Vulnerable Code ////////////////////
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

    // This function unsafely writes input bytes into the internal buffer.
    // It does not verify that the input length is within the bounds of the buffer,
    // which can lead to out-of-bounds writes (overwriting the "magic" field).
    pub fn operate(&mut self, input: &[u8]) -> Result<(), &'static str> {
        unsafe {
            let dest = self.buffer.as_mut_ptr();
            // Vulnerability: Loop does not check whether input.len() exceeds buffer size.
            for i in 0..input.len() {
                // Out-of-bounds write possible if input.len() > self.buffer.len()
                *dest.add(i) = input[i];
            }
        }
        Ok(())
    }

    // Verifies that the "magic" field remains intact.
    pub fn check(&self) -> bool {
        self.magic == 0xDEADBEEF
    }
}

fn main() {
    // Create a new data block.
    let mut obj = Data::new();
    // Feed an input that is larger than the buffer capacity.
    let input = vec![1u8; 100];
    // Process the input (vulnerable: writes out-of-bound, corrupting the "magic" field).
    let _ = obj.operate(&input);
    // Output the result of the integrity check.
    println!("Integrity check: {}", obj.check());
}