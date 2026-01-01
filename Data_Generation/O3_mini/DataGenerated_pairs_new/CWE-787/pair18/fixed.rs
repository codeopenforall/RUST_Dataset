/* 
   This Rust program corrects the previous out‐of‐bounds write issue.
   The buffer is now safely initialized with exactly the intended number of elements,
   and any update operation checks bounds before modifying the buffer.
   This fixes the CWE-787 out‐of‐bounds vulnerability.
*/
use std::thread;

struct BufferHandler {
    buffer: Vec<u32>,
}

impl BufferHandler {
    // Properly initializes the buffer with exactly `size` elements.
    fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        // SAFE: Resize properly initializes the vector to the correct length.
        buffer.resize(size, 0);
        Self { buffer }
    }

    // Safely updates the buffer; returns an error if the index is out-of-bounds.
    fn update(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.buffer.len() {
            self.buffer[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    // Returns the sum of the buffer.
    fn aggregate(&self) -> u32 {
        self.buffer.iter().sum()
    }
}

fn process() -> u32 {
    let mut handler = BufferHandler::new(10);
    // Spawn a thread to perform the update concurrently.
    let join_handle = thread::spawn(move || {
        // Attempt to update index 10 which is now out-of-range.
        // The safe update detects the error and does not modify the buffer.
        let _ = handler.update(10, 42);
        handler.aggregate()
    });
    join_handle.join().unwrap()
}

fn main() {
    let outcome = process();
    println!("Result: {}", outcome);
}