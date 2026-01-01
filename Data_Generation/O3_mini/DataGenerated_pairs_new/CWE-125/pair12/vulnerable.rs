/* 
   This Rust program implements a data handler that stores a buffer.
   It provides a method that unsafely reads an element from the underlying vector
   using unchecked pointer arithmetic. This is performed in an unsafe block
   and executed in a concurrent thread. When a user provides an index equal to the
   length of the vector (or higher), the pointer addition goes past the allocated memory,
   resulting in an out-of-bounds read (CWE-125).
*/
use std::sync::Arc;
use std::thread;

struct DataHandler {
    data: Vec<u32>,
}

impl DataHandler {
    fn new(vec: Vec<u32>) -> Self {
        Self { data: vec }
    }

    // This function is intended to return the element at the requested index.
    // However, it unsafely uses unchecked pointer arithmetic.
    fn get_item(&self, index: usize) -> Result<u32, &'static str> {
        // Vulnerability: unchecked access via unsafe pointer arithmetic
        unsafe {
            let ptr = self.data.as_ptr();
            // No bounds check is performed. An index equal to or greater than the length
            // will produce an out-of-bounds read.
            Ok(*ptr.add(index))
        }
    }
}

fn main() {
    let handler = Arc::new(DataHandler::new(vec![100, 200, 300, 400]));
    let handler_clone = Arc::clone(&handler);
    let handle = thread::spawn(move || {
        // Attempt to read an element at index 4 in a 4-element vector.
        // This causes an out-of-bounds read.
        match handler_clone.get_item(4) {
            Ok(val) => println!("Read value: {}", val),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}