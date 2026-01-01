///////////////////////
// Vulnerable Version
///////////////////////
use std::thread;

struct DataHandler;

impl DataHandler {
    // This method is intended to combine the provided byte slice into a new vector,
    // appending a trailing element that should be set to zero.
    pub fn combine(&self, input: &[u8]) -> Vec<u8> {
        let len = input.len();
        // Allocate exactly as many elements as the input bytes.
        let mut output = Vec::with_capacity(len);
        unsafe {
            // Copy input data into the vector's uninitialized memory.
            std::ptr::copy_nonoverlapping(input.as_ptr(), output.as_mut_ptr(), len);
            // OFF-BY-ONE Error: The vector's length is set to len + 1,
            // making the last element uninitialized.
            output.set_len(len + 1);
        }
        output
    }
}

fn main() {
    // Spawn a thread to simulate concurrent processing.
    let handler = DataHandler;
    let input = b"test";
    let handle = thread::spawn(move || {
        let result = handler.combine(input);
        // In this vulnerable version, the extra element remains uninitialized.
        println!("Combined output: {:?}", result);
    });
    handle.join().unwrap();
}