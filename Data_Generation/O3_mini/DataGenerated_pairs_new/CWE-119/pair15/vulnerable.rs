///////////////////////////////////////////////////////////////
// This is the vulnerable Rust code demonstrating an unsound
// memory access vulnerability due to improper bounds checking.
// The code uses unsafe pointer arithmetic inside a concurrent
// environment, mimicking real-world issues as reported in CWE-119.
///////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

trait Operations {
    fn update(&self, index: usize, value: i32);
}

struct Buffer {
    data: Mutex<Vec<i32>>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Self {
            data: Mutex::new(vec![0; size]),
        }
    }
}

impl Operations for Buffer {
    fn update(&self, index: usize, value: i32) {
        // Acquire exclusive access to the underlying vector.
        let mut guard = self.data.lock().unwrap();
        // Unsafe block: no bounds checking is performed
        // and pointer arithmetic is used to write outside
        // the allocated memory if index is invalid.
        unsafe {
            let ptr = guard.as_mut_ptr();
            // POTENTIAL FLAW: Writing to an unchecked offset
            // can result in out-of-bounds memory access.
            *ptr.offset(index as isize) = value;
        }
    }
}

fn main() {
    // Create a shared buffer with 10 elements.
    let buf = Arc::new(Buffer::new(10));
    
    // Spawn multiple threads to simulate concurrent access.
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let b = Arc::clone(&buf);
            thread::spawn(move || {
                // This deliberately triggers an out-of-bounds write,
                // since index 15 is outside the valid range.
                b.update(15, 99);
            })
        })
        .collect();

    // Join threads.
    for handle in handles {
        // In a real scenario, this may lead to undefined behavior.
        handle.join().unwrap();
    }

    // Output the current buffer contents.
    let guard = buf.data.lock().unwrap();
    println!("Buffer contents: {:?}", *guard);
}