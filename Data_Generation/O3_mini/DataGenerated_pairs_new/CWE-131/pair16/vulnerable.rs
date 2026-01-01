///////////////////////////////////////////////////////////////
// Vulnerable Code Sample for Incorrect Buffer Size Calculation
///////////////////////////////////////////////////////////////
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::str;

struct Data {
    ptr: *mut u8,
    size: usize,
}

impl Data {
    // Constructs a new Data holding a copy of the input string;
    // Vulnerability: using mem::size_of_val on the &str reference instead 
    // of using the actual input byte length.
    fn new(input: &str) -> Self {
        unsafe {
            // BUG: This computes the size as the sizeof the reference (likely 16 bytes on 64-bit)
            // rather than the length of the input string.
            let buf_size = std::mem::size_of_val(&input); 
            let layout = Layout::from_size_align(buf_size, 1).unwrap();
            let raw_ptr = alloc(layout);
            if raw_ptr.is_null() {
                panic!("Allocation failed");
            }
            // Copy only 'buf_size' bytes from the input to the buffer.
            ptr::copy_nonoverlapping(input.as_ptr(), raw_ptr, buf_size);
            Data { ptr: raw_ptr, size: buf_size }
        }
    }
    
    // Returns the content as a string slice (may be truncated if input length > size)
    fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(std::slice::from_raw_parts(self.ptr, self.size))
        }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.size, 1).unwrap();
            dealloc(self.ptr, layout);
        }
    }
}

fn main() {
    // This input exceeds the pointer size (likely 16 bytes) so the copy is truncated.
    let input = "This is a somewhat long input string causing miscalculation.";
    let stored = Data::new(input);
    println!("Stored content: '{}'", stored.as_str());
}