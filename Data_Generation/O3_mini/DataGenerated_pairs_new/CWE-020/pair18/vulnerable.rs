/* 
   This Rust program demonstrates a flaw in input validation. It holds a sensitive byte array 
   and provides a function (process) to extract a substring based on user-supplied offset and length.
   The function uses an unsafe block to perform pointer arithmetic and converts the slice into a string
   with from_utf8_unchecked without verifying that the offset and length fall within the allocated memory.
   Such improper bounds checking can lead to undefined behavior including reading beyond allocated memory.
*/
#![allow(dead_code)]
use std::env;

struct Holder {
    bytes: Box<[u8]>,
}

impl Holder {
    fn new(input: &[u8]) -> Option<Holder> {
        Some(Holder { bytes: input.into() })
    }

    // The flaw is in this method: it does not validate that offset + count is within
    // the internal byte slice length.
    fn process(&self, offset: usize, count: usize) -> String {
        unsafe {
            // Vulnerability: no check that offset+count <= self.bytes.len()
            let ptr = self.bytes.as_ptr().add(offset);                           // [Line 22]
            let slice = std::slice::from_raw_parts(ptr, count);                    // [Line 23]
            std::str::from_utf8_unchecked(slice).to_string()                       // [Line 24]
        }
    }
}

fn main() {
    // Create a Holder with a known ASCII string.
    // For example, the byte buffer holds "ABCDEFGHIJKLMNOPQRSTUVWXYZ" (26 bytes).
    let data = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let holder = Holder::new(data).expect("Initialization failed");

    // Simulate user-supplied input values that are not properly validated.
    // Here offset is 20 and count is 10 so that 20+10=30 > 26, triggering the vulnerability.
    let user_offset = 20;
    let user_count = 10;

    // Process the extraction with unsafe boundaries.
    let result = holder.process(user_offset, user_count);
    println!("Extracted result: {}", result);
}