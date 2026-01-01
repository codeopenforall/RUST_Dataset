/* 
   This Rust program implements the same data handler but with a secure fix.
   The get_item method now checks the index against the vector length before attempting
   any access, thereby preventing any out-of-bounds read.
   The code also retains the concurrent access pattern to mimic real usage.
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

    // Securely retrieves an element by verifying that the index is within bounds.
    fn get_item(&self, index: usize) -> Result<u32, &'static str> {
        if index < self.data.len() {
            // Safe access via slice indexing.
            Ok(self.data[index])
        } else {
            // Prevent out-of-bounds access by returning an error.
            Err("Index out-of-range")
        }
    }
}

fn main() {
    let handler = Arc::new(DataHandler::new(vec![100, 200, 300, 400]));
    let handler_clone = Arc::clone(&handler);
    let handle = thread::spawn(move || {
        // Attempt to read an element at index 4 in a 4-element vector.
        // The fix prevents an unsafe read and returns an error.
        match handler_clone.get_item(4) {
            Ok(val) => println!("Read value: {}", val),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}