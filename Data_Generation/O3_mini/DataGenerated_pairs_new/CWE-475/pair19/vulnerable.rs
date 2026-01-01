///////////////////////////////////////////////
// WARNING: This code exhibits undefined behavior 
// CWE-475: Undefined Behavior Through Unsound API Use
///////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct Handler {
    buffer: *const u8,
    size: usize,
}

impl Handler {
    fn new(data: &[u8]) -> Self {
        Handler {
            buffer: data.as_ptr(),
            size: data.len(),
        }
    }

    // This unsafe routine improperly calculates the number of bytes to copy.
    // It offsets the source pointer by 1 but then copies "size" bytes, which
    // overruns the allocated memory when size equals data.len(). This violates the
    // API preconditions of ptr::copy_nonoverlapping.
    unsafe fn operate(&self) -> Vec<u8> {
        let mut output = vec![0u8; self.size];
        let offset_ptr = self.buffer.offset(1);  // Vulnerable: Assumes buffer is long enough.
        ptr::copy_nonoverlapping(offset_ptr, output.as_mut_ptr(), self.size);
        output
    }
}

fn main() {
    let data = vec![10, 20, 30, 40, 50];
    let handler = Handler::new(&data);

    // Simulate a concurrent context.
    let shared_val = Arc::new(Mutex::new(0));
    let shared_copy = Arc::clone(&shared_val);
    let joiner = thread::spawn(move || {
         let mut val = shared_copy.lock().unwrap();
         *val += 1;
    });

    unsafe {
         // Undefined behavior: copying 5 bytes starting from data[1] on a 5-byte buffer.
         let result = handler.operate();
         println!("Output: {:?}", result);
    }
    joiner.join().unwrap();
}