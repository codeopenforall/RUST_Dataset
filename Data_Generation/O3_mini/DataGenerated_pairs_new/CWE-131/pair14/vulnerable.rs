////////////////////////////////////////
// Vulnerability demonstration code  //
// CWE-131: Incorrect Calculation of   //
// Buffer Size (using wrong size metric)//
////////////////////////////////////////
use std::mem;
use std::ptr;
use std::thread;

pub struct Handler;

impl Handler {
    // Copies the incoming string into a byte buffer.
    // Mistakenly uses mem::size_of_val(&input) (the size of the fat pointer)
    // instead of the actual string length.
    pub fn process(input: &str) -> Vec<u8> {
        unsafe {
            // Vulnerability: calculates size using fat pointer metadata.
            let size = mem::size_of_val(&input); // Incorrect: returns constant size (e.g. 16 on 64-bit)
            let mut buffer = Vec::with_capacity(size);
            // Unsafely set the length to the allocated capacity.
            buffer.set_len(size);
            // Copies 'size' bytes from the input.
            ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), size);
            buffer
        }
    }
}

fn main() {
    let input = "This is a test string that exceeds the typical pointer size";
    // Using thread concurrency to mimic a real-world scenario.
    let handle = thread::spawn(move || {
        let result = Handler::process(input);
        // The following assertion is expected to fail because the buffer copy is truncated.
        assert_eq!(result, input.as_bytes(), "Buffer copy failed: data mismatch");
    });
    handle.join().expect("Thread panicked");
}