use std::mem;
use std::ptr;
use std::thread;
use std::sync::Arc;

struct Handler;

impl Handler {
    // This method duplicates an input string by copying its bytes
    // into a newly allocated buffer. However, it miscalculates
    // the buffer size by using the size of a string reference rather than
    // the actual length of the string.
    fn replicate(&self, input: &str) -> String {
        // Incorrect buffer size computation: size_of::<&str>() returns the size
        // of the pointer data (typically 16 bytes on 64-bit systems) instead of input.len()
        let required = mem::size_of::<&str>(); // Vulnerable calculation
        let mut buffer = Vec::with_capacity(required);
        unsafe {
            buffer.set_len(required);
            // Unsafe copy: copies "required" bytes from input, which may be either
            // too many or too few bytes compared to input.len(), risking out-of-bound read.
            ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), required);
        }
        // Converting the possibly invalid UTF-8 bytes to a Rust String (lossy conversion)
        // which might hide some corruption, but the string will not match the input.
        String::from_utf8_lossy(&buffer).into()
    }
}

fn main() {
    let handler = Arc::new(Handler);
    let data = "Hello, Rust!";
    let h = handler.clone();
    let joiner = thread::spawn(move || {
        h.replicate(data)
    });
    let result = joiner.join().unwrap();
    println!("{}", result);
}