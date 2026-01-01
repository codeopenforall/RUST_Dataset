use std::mem;
use std::ptr;
use std::thread;
use std::sync::Arc;

struct Handler;

impl Handler {
    // This corrected method properly calculates the buffer size using input.len(),
    // ensuring that exactly the number of bytes representing the string are copied.
    fn replicate(&self, input: &str) -> String {
        let required = input.len(); // Correct calculation using the actual length of the input string.
        let mut buffer = Vec::with_capacity(required);
        unsafe {
            buffer.set_len(required);
            ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), required);
        }
        // Safe conversion to String is now guaranteed because we copied exactly the valid UTF-8 bytes.
        String::from_utf8(buffer).expect("Invalid UTF-8 sequence")
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