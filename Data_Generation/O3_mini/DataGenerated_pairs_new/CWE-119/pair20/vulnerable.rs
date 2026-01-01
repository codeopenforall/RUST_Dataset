/////////////////////////////////////////////////////////////
// This code implements a buffer processor which accepts an
// input byte slice and copies it into an internal fixed-size
// buffer using unsafe pointer arithmetic. However, no check
// is performed to ensure that the input length does not exceed
// the buffer’s capacity, leading to a potential buffer overflow.
/////////////////////////////////////////////////////////////

use std::ptr;

struct BufferProcessor {
    buffer: [u8; 10],
}

impl BufferProcessor {
    fn new() -> Self {
        BufferProcessor { buffer: [0; 10] }
    }

    fn update(&mut self, input: &[u8]) -> Result<(), &'static str> {
        // Vulnerable: no bounds verification on input length.
        unsafe {
            let dest = self.buffer.as_mut_ptr();
            // Loop copies input into the internal buffer.
            for i in 0..input.len() {
                // ISSUE (CWE-119): Writing without verifying that i is within bounds.
                ptr::write(dest.add(i), input[i]);
            }
        }
        Ok(())
    }

    fn read(&self) -> &[u8] {
        &self.buffer
    }
}

fn main() {
    let mut processor = BufferProcessor::new();
    // This input is longer than 10 bytes, causing an out-of-bound write.
    let data = b"ABCDEFGHIJKL"; // 12 bytes -> potential buffer overflow
    match processor.update(data) {
        Ok(()) => println!("Content: {:?}", processor.read()),
        Err(e) => println!("Error: {}", e),
    }
}