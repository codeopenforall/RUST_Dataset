////////////////////////////////////////////////////////////////////////////////////////////////////
// Note: This sample uses unsafe dynamic allocation via std::alloc and mis‐calculates the buffer
// size by using std::mem::size_of_val on the string slice reference rather than its byte length.
// This leads to a buffer that is too small when the input string is longer than the pointer size 
// (commonly 16 bytes on 64‑bit platforms). In concurrent or performance‐sensitive contexts this 
// pattern has been observed in real‑world Rust code vulnerable to buffer miscalculation errors (CWE‑131).
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::slice;

struct Holder {
    ptr: *mut u8,
    len: usize,
}

impl Holder {
    // Constructs a new Holder instance by allocating a buffer.
    // BUG: The buffer size is computed as size_of_val(&input) rather than input.len().
    fn new(input: &str) -> Self {
        // Incorrect calculation: size_of_val returns the size of the reference, not of the string
        let requested = mem::size_of_val(&input);
        let layout = Layout::from_size_align(requested, 1).unwrap();
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        // Copy as many bytes as fit in the allocated buffer.
        // If input.len() > requested, only a partial copy will occur.
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), ptr, input.len().min(requested));
        }
        Self { ptr, len: requested }
    }
    
    // Retrieves a string slice from the internal buffer.
    // Note: May yield incomplete data if input was truncated.
    fn as_str(&self) -> &str {
        unsafe {
            let data = slice::from_raw_parts(self.ptr, self.len);
            std::str::from_utf8(data).unwrap_or("")
        }
    }
}

impl Drop for Holder {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.len, 1).unwrap();
        unsafe { dealloc(self.ptr, layout) };
    }
}

// Interface function processing the input.
fn process_input(input: &str) -> String {
    let instance = Holder::new(input);
    instance.as_str().to_owned()
}

fn main() {
    let input = "This is a string longer than sixteen!";
    let output = process_input(input);
    println!("Processed: {}", output);
}