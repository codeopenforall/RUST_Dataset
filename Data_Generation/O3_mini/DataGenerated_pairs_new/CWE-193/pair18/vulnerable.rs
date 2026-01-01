////////////////////////////////////////
// Vulnerable Rust Code for Off-by-One
////////////////////////////////////////
use std::ptr;

struct DataHandler;

impl DataHandler {
    // This function copies data from the input slice into a newly allocated vector.
    // It uses an unsafe block to perform raw pointer writes and then sets the length manually.
    // The off-by-one error occurs when it sets the length to input.len() + 1.
    pub fn process(&self, input: &[u8]) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(input.len());
        unsafe {
            let dest = buffer.as_mut_ptr();
            // Copy each element from the input slice into the buffer.
            for i in 0..input.len() {
                // Copy element using unchecked access
                let value = *input.get_unchecked(i);
                ptr::write(dest.add(i), value);
            }
            // Vulnerability: Off-by-one error when marking the vector as initialized.
            // The length is mistakenly set to input.len() + 1 which writes beyond the allocated memory.
            buffer.set_len(input.len() + 1);
        }
        buffer
    }
}

fn main() {
    let handler = DataHandler;
    // Example input that triggers the off-by-one error.
    let sample = vec![10, 20, 30, 40];
    let result = handler.process(&sample);
    println!("Processed output: {:?}", result);
}