////////////////////////////////////////
// Corrected demonstration code       //
// Properly calculates buffer size by   //
// using the string length              //
////////////////////////////////////////
use std::ptr;
use std::thread;

pub struct Handler;

impl Handler {
    // Copies the incoming string into a byte buffer.
    // Correctly uses input.len() to compute the exact size needed.
    pub fn process(input: &str) -> Vec<u8> {
        unsafe {
            // Correct computation: use the true length of the string.
            let size = input.len();
            let mut buffer = Vec::with_capacity(size);
            // Unsafely set the length to the allocated capacity.
            buffer.set_len(size);
            // Copies the full input into the buffer.
            ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), size);
            buffer
        }
    }
}

fn main() {
    let input = "This is a test string that exceeds the typical pointer size";
    // Using thread concurrency as in the vulnerable version.
    let handle = thread::spawn(move || {
        let result = Handler::process(input);
        // The assertion passes as the full data is correctly copied.
        assert_eq!(result, input.as_bytes(), "Buffer copy failed: data mismatch");
    });
    handle.join().expect("Thread panicked");
}